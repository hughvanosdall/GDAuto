//! Reading the two MSVC C++ types Grim Dawn hands back across the export
//! boundary: `std::string` and `mem::vector<T>`.
//!
//! Everywhere else the project resolves things by name and reads them through
//! the game's own accessors, with no layout knowledge at all. These two are the
//! exception, and they are a *much* safer exception than a struct offset:
//!
//! * the layouts are the MSVC standard library's, not Grim Dawn's, so they do
//!   not move when Crate recompiles the game — only if Microsoft changes the
//!   STL ABI, which they have not done since VS2015;
//! * every read is validated before it is trusted, and an implausible value
//!   yields `None` rather than a wrong answer or a fault.
//!
//! Nothing here writes. The worst a bug can do is fail to read a string.

/// Longest string we will pull out of the game. Skill tags are ~30 characters;
/// anything near this is a sign the layout assumption is wrong.
const MAX_STRING: usize = 4096;

/// Most elements we will walk in a vector. The player's skill list is a few
/// hundred.
const MAX_VECTOR: usize = 100_000;

/// MSVC's `std::basic_string<char>`:
///
/// ```text
///   +0x00  union { char buf[16]; char* ptr; }   small-string buffer
///   +0x10  size_t size
///   +0x18  size_t capacity
/// ```
///
/// A string is stored inline when `capacity < 16`, and behind `ptr` otherwise.
/// That switch is the whole trick, and getting it backwards yields a pointer
/// read as text or text read as a pointer -- hence the checks.
///
/// # Safety
/// `ptr` must point at a live `std::string` owned by the game, and this must be
/// read on the frame-hook thread, before anything can reallocate it.
pub unsafe fn read_string(ptr: *const c_void) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    let base = ptr as *const u8;
    let size = std::ptr::read_unaligned(base.add(0x10) as *const usize);
    let capacity = std::ptr::read_unaligned(base.add(0x18) as *const usize);

    // A live string always has capacity >= size, and neither is enormous.
    if size > capacity || capacity > MAX_STRING {
        return None;
    }
    if size == 0 {
        return Some(String::new());
    }

    let data: *const u8 = if capacity < 16 {
        base // small-string optimisation: the bytes are here
    } else {
        let heap = std::ptr::read_unaligned(base as *const *const u8);
        if heap.is_null() {
            return None;
        }
        heap
    };

    let bytes = std::slice::from_raw_parts(data, size);
    // Record paths and tags are ASCII; anything else means a bad read.
    if !bytes.iter().all(|b| b.is_ascii_graphic() || *b == b' ' || *b == b'_') {
        return None;
    }
    Some(String::from_utf8_lossy(bytes).into_owned())
}

/// `mem::vector<T>`, which has the same three-pointer shape as `std::vector`:
///
/// ```text
///   +0x00  T* first
///   +0x08  T* last        (one past the end)
///   +0x10  T* end_of_capacity
/// ```
///
/// Returns the elements as a slice. `None` if the three pointers do not
/// describe a sane, correctly aligned, non-absurd range.
///
/// # Safety
/// As [`read_string`]. The slice borrows the game's memory and must not
/// outlive the tick that read it.
pub unsafe fn read_vector<'a, T>(ptr: *const c_void) -> Option<&'a [T]> {
    if ptr.is_null() {
        return None;
    }
    let base = ptr as *const *const u8;
    let first = std::ptr::read_unaligned(base);
    let last = std::ptr::read_unaligned(base.add(1));
    let cap_end = std::ptr::read_unaligned(base.add(2));

    if first.is_null() {
        // An empty vector is legitimately all-null.
        return (last.is_null() && cap_end.is_null()).then_some(&[]);
    }
    if last < first || cap_end < last {
        return None;
    }

    let span = last as usize - first as usize;
    let stride = std::mem::size_of::<T>();
    if stride == 0 || span % stride != 0 {
        return None;
    }
    let count = span / stride;
    if count > MAX_VECTOR {
        return None;
    }
    if (first as usize) % std::mem::align_of::<T>() != 0 {
        return None;
    }

    Some(std::slice::from_raw_parts(first as *const T, count))
}

use std::ffi::c_void;

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a byte image of an MSVC `std::string` and read it back. This does
    /// not prove the game uses this layout -- only a live read can -- but it
    /// does pin the reader's own behaviour, including which branch it takes.
    fn make_string(text: &str, heap: Option<&mut Vec<u8>>) -> Vec<u8> {
        let mut buf = vec![0u8; 0x20];
        let size = text.len();
        match heap {
            None => {
                // Inline: capacity 15, bytes at offset 0.
                buf[..size].copy_from_slice(text.as_bytes());
                buf[0x10..0x18].copy_from_slice(&size.to_le_bytes());
                buf[0x18..0x20].copy_from_slice(&15usize.to_le_bytes());
            }
            Some(storage) => {
                storage.clear();
                storage.extend_from_slice(text.as_bytes());
                let ptr = storage.as_ptr() as usize;
                buf[0..8].copy_from_slice(&ptr.to_le_bytes());
                buf[0x10..0x18].copy_from_slice(&size.to_le_bytes());
                buf[0x18..0x20].copy_from_slice(&(size + 1).to_le_bytes());
            }
        }
        buf
    }

    #[test]
    fn reads_a_short_string_from_the_inline_buffer() {
        let image = make_string("tagClass03", None);
        let got = unsafe { read_string(image.as_ptr() as *const c_void) };
        assert_eq!(got.as_deref(), Some("tagClass03"));
    }

    #[test]
    fn reads_a_long_string_from_the_heap() {
        let text = "records/skills/playerclass03/bloodofdreeg1.dbr";
        let mut heap = Vec::new();
        let image = make_string(text, Some(&mut heap));
        let got = unsafe { read_string(image.as_ptr() as *const c_void) };
        assert_eq!(got.as_deref(), Some(text));
    }

    #[test]
    fn an_empty_string_is_empty_not_a_failure() {
        let image = make_string("", None);
        assert_eq!(unsafe { read_string(image.as_ptr() as *const c_void) }.as_deref(), Some(""));
    }

    #[test]
    fn nonsense_headers_are_refused() {
        // size > capacity cannot happen in a live string.
        let mut image = vec![0u8; 0x20];
        image[0x10..0x18].copy_from_slice(&99usize.to_le_bytes());
        image[0x18..0x20].copy_from_slice(&4usize.to_le_bytes());
        assert!(unsafe { read_string(image.as_ptr() as *const c_void) }.is_none());

        // An absurd capacity is the signature of reading the wrong address.
        let mut image = vec![0u8; 0x20];
        image[0x10..0x18].copy_from_slice(&8usize.to_le_bytes());
        image[0x18..0x20].copy_from_slice(&usize::MAX.to_le_bytes());
        assert!(unsafe { read_string(image.as_ptr() as *const c_void) }.is_none());
    }

    #[test]
    fn a_null_pointer_is_none_rather_than_a_fault() {
        assert!(unsafe { read_string(std::ptr::null()) }.is_none());
        assert!(unsafe { read_vector::<u64>(std::ptr::null()) }.is_none());
    }

    #[test]
    fn reads_a_vector_of_pointers() {
        let items: Vec<u64> = vec![10, 20, 30, 40];
        let image: [usize; 3] = [
            items.as_ptr() as usize,
            unsafe { items.as_ptr().add(items.len()) } as usize,
            unsafe { items.as_ptr().add(items.len()) } as usize,
        ];
        let got = unsafe { read_vector::<u64>(image.as_ptr() as *const c_void) }.unwrap();
        assert_eq!(got, &[10, 20, 30, 40]);
    }

    #[test]
    fn an_empty_vector_reads_as_empty() {
        let image: [usize; 3] = [0, 0, 0];
        let got = unsafe { read_vector::<u64>(image.as_ptr() as *const c_void) };
        assert_eq!(got, Some(&[][..]));
    }

    #[test]
    fn a_backwards_vector_is_refused() {
        let items: Vec<u64> = vec![1, 2, 3];
        let image: [usize; 3] = [
            unsafe { items.as_ptr().add(3) } as usize,
            items.as_ptr() as usize, // last < first
            unsafe { items.as_ptr().add(3) } as usize,
        ];
        assert!(unsafe { read_vector::<u64>(image.as_ptr() as *const c_void) }.is_none());
    }

    #[test]
    fn a_misaligned_span_is_refused() {
        let items: Vec<u8> = vec![0; 32];
        // A span of 7 bytes cannot be a whole number of u64s.
        let image: [usize; 3] = [
            items.as_ptr() as usize,
            unsafe { items.as_ptr().add(7) } as usize,
            unsafe { items.as_ptr().add(32) } as usize,
        ];
        assert!(unsafe { read_vector::<u64>(image.as_ptr() as *const c_void) }.is_none());
    }
}
