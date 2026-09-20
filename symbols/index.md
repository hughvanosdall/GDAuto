# Grim Dawn exported symbol index

Generated 2026-09-20T22:39:06+00:00 from `C:\Program Files (x86)\Steam\steamapps\common\Grim Dawn\x64`.

Do not edit by hand -- regenerate with `python tools/dump_exports.py`.
Search with `python tools/query_symbols.py`.

## Modules

| Module | Machine | Exports | Built (UTC) | SHA-256 |
|---|---|---:|---|---|
| `Game.dll` | x64 | 25,100 | 2026-08-19 | `07775a297050e84a…` |
| `Engine.dll` | x64 | 6,283 | 2026-08-19 | `9f2042e1fba6c926…` |
| `Widget.dll` | x64 | 262 | 2026-08-19 | `4cdee48b5b42f920…` |

## Largest classes

| Class | Module | Members |
|---|---|---:|
| `GameEngine` | Game.dll | 809 |
| `Character` | Game.dll | 714 |
| `Skill` | Game.dll | 422 |
| `Player` | Game.dll | 297 |
| `Engine` | Engine.dll | 238 |
| `SkillManager` | Game.dll | 212 |
| `ControllerMonster` | Game.dll | 196 |
| `Item` | Game.dll | 181 |
| `Entity` | Engine.dll | 165 |
| `Actor` | Engine.dll | 161 |
| `SkillProfile` | Game.dll | 160 |
| `Monster` | Game.dll | 149 |
| `GraphicsMeshInstance` | Engine.dll | 128 |
| `GraphicsMTRenderer` | Engine.dll | 123 |
| `GameEngineInboundInterface` | Game.dll | 119 |
| `GameEngineOutboundInterface` | Game.dll | 119 |
| `ControllerAIStateT<GAME::ControllerAI,GAME::Character>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerCerberus,GAME::Cerberus>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Megalesios>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerMonster,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerMonsterHidden,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerMonsterSynergy,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerNpc2,GAME::Npc>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Character>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Npc>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerNpcHerder,GAME::Character>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerPlayer,GAME::Player>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerSpirit,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerSpiritHost,GAME::SpiritHost>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerStationaryMonster,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerTerracotta,GAME::Monster>` | Game.dll | 118 |
| `ControllerAIStateT<GAME::ControllerTyphonChained,GAME::Monster>` | Game.dll | 118 |
| `ControllerAI` | Game.dll | 114 |
| `GraphicsEngine` | Engine.dll | 112 |
| `World` | Engine.dll | 110 |
| `ItemEquipment` | Game.dll | 108 |
| `PlayStats` | Game.dll | 101 |
| `GraphicsCanvas` | Engine.dll | 97 |
| `ControllerPlayer` | Game.dll | 95 |
| `ItemArtifactFormula` | Game.dll | 94 |
| `GAME` | Engine.dll | 86 |
| `SoundManager` | Engine.dll | 84 |
| `Terrain` | Engine.dll | 84 |
| `Region` | Engine.dll | 83 |
| `CombatManager` | Game.dll | 82 |
| `Npc` | Game.dll | 81 |
| `GameInfo` | Engine.dll | 78 |
| `PlayerInventoryCtrl` | Game.dll | 76 |
| `Level` | Engine.dll | 76 |
| `WaterType` | Engine.dll | 74 |
| `StaticShrine` | Game.dll | 72 |
| `ItemRelic` | Game.dll | 67 |
| `ConnectionManager` | Engine.dll | 67 |
| `Window` | Widget.dll | 67 |
| `ProjectileBase` | Game.dll | 66 |
| `GraphicsMesh` | Engine.dll | 65 |
| `EndlessDungeon_Generator` | Game.dll | 63 |
| `PlayerHotSlotCtrl` | Game.dll | 63 |
| `Emitter` | Engine.dll | 61 |

## All classes

### `Achievement` (Game.dll, 16)

- `Achievement`
- `Achievement`
- `GetDescription` `C`
- `GetGameId` `C`
- `GetHidden` `C`
- `GetLockedIcon` `C`
- `GetTitle` `C`
- `GetUnlockedIcon` `C`
- `IsUnlocked` `C`
- `LoadFromDatabase` `V`
- `Sync` `V`
- `Unlock`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~Achievement` `V`

### `AchievementManager` (Game.dll, 3)

- `GetGroups` `C`
- `Reset`
- `Unlock`

### `AckEntityPacket` (Game.dll, 8)

- `AckEntityPacket`
- `AckEntityPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AckEntityPacket` `V`

### `ActivateAltarPacket` (Game.dll, 8)

- `ActivateAltarPacket`
- `ActivateAltarPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ActivateAltarPacket` `V`

### `ActivateSkillConfigCmd` (Game.dll, 7)

- `ActivateSkillConfigCmd`
- `ActivateSkillConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~ActivateSkillConfigCmd` `V`

### `ActivateSkillConfigCmdPacket` (Game.dll, 8)

- `ActivateSkillConfigCmdPacket`
- `ActivateSkillConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ActivateSkillConfigCmdPacket` `V`

### `ActivityManager` (Game.dll, 11)

- `Clear`
- `Destroy` `S`
- `FastInstallActivity`
- `Get` `S`
- `GetActivity`
- `InstallActivity`
- `IsActivityActive` `C`
- `IsAlreadyTeleporting` `C`
- `Render` `C`
- `Update`
- `~ActivityManager`

### `Actor` (Engine.dll, 161)

- `Actor`
- `AddToScene` `V`
- `AnimationCallback` `V`
- `AppendDetailMapData` `V`
- `Attach` `V`
- `Attach` `V`
- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `CheckLOS` `VC`
- `ClearAnimationCallbacks`
- `CollisionCallback` `V`
- `CreateAttachmentsFromMesh`
- `CreatePathObstacles` `V`
- `Detach` `V`
- `DisableCreateAttachements`
- `EnableOutline` `V`
- `Enqueue` `V`
- `FastUpdate` `V`
- `ForceDefaultMeshFile`
- `ForcePoseUpdate` `V`
- `GeometryBusStop` `V`
- `GeometryBusStop` `V`
- `GetAlternateMeshName` `VC`
- `GetAmbientHighlight` `VC`
- `GetAnimationSpeedMultiplier` `C`
- `GetAttachedCoords` `VC`
- `GetAttachedCoordsInRegion` `VC`
- `GetBaseScale` `C`
- `GetBaseTextureName` `C`
- `GetBaseTextureName` `C`
- `GetBaseTexturesSize` `C`
- `GetBoneCoords` `C`
- `GetBoneCoordsInRegion` `VC`
- `GetBumpTextureName` `C`
- `GetBumpTextureName` `C`
- `GetBumpTexturesSize` `C`
- `GetCastsShadows` `C`
- `GetCenterOfMass` `VC`
- `GetCollisionBox` `VC`
- `GetCollisionShape` `VC`
- `GetCurrentScale` `C`
- `GetCustomNuggetBitmap` `C`
- `GetDbrShaderName` `C`
- `GetDescriptionTag` `C`
- `GetDetailTextureName` `C`
- `GetDetailTextureName` `C`
- `GetDetailTexturesSize` `C`
- `GetExtents` `VC`
- `GetFellFaceDown` `C`
- `GetGameDescription` `VC`
- `GetGlowTextureName` `C`
- `GetGlowTextureName` `C`
- `GetGlowTexturesSize` `C`
- `GetHeight` `C`
- `GetHighlight` `C`
- `GetHitBox` `VC`
- `GetHitBox` `VC`
- `GetInteractIconOffset` `C`
- `GetInteractIconOverride` `C`
- `GetIntersection` `VC`
- `GetIsDissolved` `C`
- `GetLastFrameRendered` `C`
- `GetMeshFileName` `C`
- `GetMeshInstance` `VC`
- `GetNormal` `VC`
- `GetNumHitBoxes` `VC`
- `GetNumSkeletonEmitterBones` `C`
- `GetParentActor` `C`
- `GetPhysicsFriction` `VC`
- `GetPhysicsMass` `VC`
- `GetPhysicsMesh` `VC`
- `GetPhysicsRestitution` `VC`
- `GetPose` `V`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetSkeletonEmitterBone` `C`
- `GetSpecTextureName` `C`
- `GetSpecTextureName` `C`
- `GetSpecTexturesSize` `C`
- `GetStaticClassInfo` `S`
- `GetTintColor` `C`
- `GetTransparency` `C`
- `GetUICloseDistance` `VC`
- `GetVisibility` `VC`
- `GetWorldDescOffset` `C`
- `HandleAnimationCallbacks`
- `HasAttachPoint` `C`
- `HasBone` `C`
- `HasRigidBodyData` `C`
- `InitialUpdate` `V`
- `IsCharacter` `V`
- `IsDescriptionVisible` `VC`
- `Load` `V`
- `LoadLite` `V`
- `LocalEnqueue` `V`
- `OccludesPathing` `VC`
- `OnAddToLevel` `V`
- `OnDestroy` `V`
- `OnMoveInLevel` `V`
- `OnRemoveFromLevel` `V`
- `OnTeleported` `V`
- `PhysicsPost` `V`
- `PhysicsSetup` `V`
- `PhysicsUpdate` `V`
- `PlayAnimation` `V`
- `PlaySound`
- `PreAnimationUpdate` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Read` `V`
- `RegisterAnimationCallback` `V`
- `RemoveAttachmentsFromMesh`
- `RemovePathObstacles` `V`
- `ResetCreateAttachments`
- `RestoreBaseTextures`
- `RestoreBumpTextures`
- `RestoreGlowTextures`
- `RestoreMesh`
- `RestoreSpecTextures`
- `RestoreState` `V`
- `SaveState` `VC`
- `SendCameraShakeEvent` `V`
- `SetAnimationSpeedMultiplier`
- `SetBaseTexture`
- `SetBumpTexture`
- `SetCastsShadows`
- `SetDescriptionVisible`
- `SetDetailTexture`
- `SetFellFaceDown`
- `SetGlowTexture`
- `SetHighlight` `V`
- `SetMesh`
- `SetMeshInstance`
- `SetObjectSpacePose`
- `SetOutlineColor`
- `SetParentLevel` `V`
- `SetPose` `V`
- `SetScale` `V`
- `SetScale` `V`
- `SetSpecTexture`
- `SetTintColor`
- `SetTransparency`
- `SetTransparency` `V`
- `SetTransparent` `V`
- `SetVisibility` `V`
- `SetVisibility` `V`
- `ShouldCastShadows` `VC`
- `ShouldRenderForScene` `VC`
- `StopAnimation`
- `StopAnimations`
- `TweakPose` `V`
- `UpdateBoundingBox` `V`
- `UpdatePose`
- `UpdateSelf` `V`
- `UseAlternateMesh` `VC`
- `Write` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Actor` `V`

### `ActorConfigCommand` (Engine.dll, 9)

- `ActorConfigCommand`
- `ActorConfigCommand`
- `Execute` `V`
- `GetNetPacket` `V`
- `GetParentId` `C`
- `SupportsNetwork` `C`
- ``vftable'`
- `operator=`
- `~ActorConfigCommand` `V`

### `AddInventoryItemConfigCmd` (Game.dll, 7)

- `AddInventoryItemConfigCmd`
- `AddInventoryItemConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~AddInventoryItemConfigCmd` `V`

### `AddInventoryItemConfigCmdPacket` (Game.dll, 8)

- `AddInventoryItemConfigCmdPacket`
- `AddInventoryItemConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AddInventoryItemConfigCmdPacket` `V`

### `AddPetBonusFxPakPacket` (Game.dll, 8)

- `AddPetBonusFxPakPacket`
- `AddPetBonusFxPakPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AddPetBonusFxPakPacket` `V`

### `AdjustFactionConfigCmd` (Game.dll, 7)

- `AdjustFactionConfigCmd`
- `AdjustFactionConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~AdjustFactionConfigCmd` `V`

### `AdjustFactionConfigCmdPacket` (Game.dll, 8)

- `AdjustFactionConfigCmdPacket`
- `AdjustFactionConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AdjustFactionConfigCmdPacket` `V`

### `AdjustmentSectorData` (Engine.dll, 8)

- `AdjustmentSectorData`
- `AdjustmentSectorData`
- `AdjustmentSectorData`
- `Copy` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~AdjustmentSectorData` `V`

### `AllPlayersBasicInfoPacket` (Game.dll, 8)

- `AllPlayersBasicInfoPacket`
- `AllPlayersBasicInfoPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AllPlayersBasicInfoPacket` `V`

### `AllPlayersHeartbeatPacket` (Game.dll, 8)

- `AllPlayersHeartbeatPacket`
- `AllPlayersHeartbeatPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AllPlayersHeartbeatPacket` `V`

### `AltarOfferConfigCmd` (Game.dll, 8)

- `AltarOfferConfigCmd`
- `AltarOfferConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeOfferings`
- ``vftable'`
- `operator=`
- `~AltarOfferConfigCmd` `V`

### `AltarOfferConfigCmdPacket` (Game.dll, 8)

- `AltarOfferConfigCmdPacket`
- `AltarOfferConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AltarOfferConfigCmdPacket` `V`

### `AltarReagentsRequestPacket` (Game.dll, 8)

- `AltarReagentsRequestPacket`
- `AltarReagentsRequestPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AltarReagentsRequestPacket` `V`

### `AltarStateChangeConfigCmd` (Game.dll, 7)

- `AltarStateChangeConfigCmd`
- `AltarStateChangeConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~AltarStateChangeConfigCmd` `V`

### `AltarStateChangeConfigCmdPacket` (Game.dll, 8)

- `AltarStateChangeConfigCmdPacket`
- `AltarStateChangeConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AltarStateChangeConfigCmdPacket` `V`

### `AmbianceManager` (Engine.dll, 35)

- `AmbianceManager`
- `EnableEnvironmentEffects`
- `ForceUpdateFog`
- `GetAdjustmentParameters` `C`
- `GetBloomParameters` `C`
- `GetDepthFog` `C`
- `GetEnvironmentEffects` `C`
- `GetFogColor` `C`
- `GetGroundAmbientColor` `C`
- `GetHeightFog` `C`
- `GetInterpolatedNightLightScale` `C`
- `GetIsDay` `C`
- `GetIsFogEnabled` `C`
- `GetLightColor` `C`
- `GetLightDirection` `C`
- `GetNightLightScale` `C`
- `GetShadowIntensity` `C`
- `GetSkyAmbientColor` `C`
- `GetTime` `C`
- `IncrementBloomParams`
- `IsTimeEnabled` `C`
- `Reset`
- `ResetAmbientMusic`
- `ResetAmbientSound`
- `SetDebug`
- `SetDepthFogClamp`
- `SetDepthFogClamping`
- `SetLightingTransformAngle`
- `SetTarget`
- `SetTime`
- `SetTimeEnabled`
- `SetWeatherAdjustments`
- `TargetIsInBossMusicZone` `C`
- `Update`
- `~AmbianceManager`

### `AmbientCharacter` (Game.dll, 14)

- `AmbientCharacter`
- `CreateUISummaryText` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSkillId1` `C`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `UnderAttack` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~AmbientCharacter` `V`

### `AmbientSectorData` (Engine.dll, 8)

- `AmbientSectorData`
- `AmbientSectorData`
- `AmbientSectorData`
- `Copy` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~AmbientSectorData` `V`

### `AmbientShadow` (Game.dll, 16)

- `AddToScene`
- `AddToScene`
- `AmbientShadow`
- `AmbientShadow`
- `GetNumRenderPasses` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetTexture` `VC`
- `LogInfo` `VC`
- `RenderPass` `VC`
- `SetTexture`
- ``vftable'`
- `operator=`
- `~AmbientShadow`

### `AnchorManager` (Widget.dll, 3)

- `AddAnchor`
- `AnchorManager`
- `Update`

### `AngerManager` (Game.dll, 25)

- `AddAnger`
- `AddAngerGraph` `C`
- `AngerManager`
- `AngerManager`
- `Clear`
- `DebugRender`
- `GetAnger` `C`
- `GetAngerDiff` `C`
- `GetCurrentTargetNotMostHated` `C`
- `GetMaxAnger` `S`
- `GetNewTarget`
- `GetShowCurrentEnemy` `C`
- `HasOnlyPetTargets` `C`
- `SetDistanceAngerComparator`
- `SetMostHatedEnemy`
- `SetParent`
- `ShouldRemoveEnemy`
- `ShowAngerLevels` `C`
- `ShowCurrentEnemyToggle` `S`
- `SubtractAnger`
- `TransferAnger` `C`
- `Update`
- `m_sbShowCurrentEnemy` `S`
- `operator=`
- `~AngerManager`

### `AnimChannel` (Engine.dll, 17)

- `AnimChannel`
- `AnimChannel`
- `AnimChannel`
- `GetBlendAmount` `C`
- `GetCurrentAnimationName` `C`
- `GetCurrentFrame` `C`
- `GetPreviousAnimationNames` `C`
- `GetRemainingAnimationTime` `C`
- `IsValid` `C`
- `PlayAnimation`
- `SetEntity`
- `StopAnimation`
- `StorePose`
- `Update`
- `operator=`
- `operator=`
- `~AnimChannel`

### `AnimationSet` (Game.dll, 28)

- `AddAnimation`
- `AdjustAnimation`
- `AnimationSet`
- `AnimationSet`
- `CalculateAllocatedMemory` `C`
- `Cancel`
- `DoesAnimationExist`
- `GetAnimSpeed` `C`
- `GetAnimTextAsType` `S`
- `GetAnimTypeAsText` `S`
- `GetAnimationBase` `C`
- `GetAnimationBase`
- `GetAnimationBase` `C`
- `GetCurrentType` `C`
- `GetFrameRate` `C`
- `GetLength` `C`
- `GetNextType` `C`
- `HandleAnimationCallback`
- `IsLooping` `C`
- `PlayAnimation`
- `PlayAnimationIfAvailable`
- `PreLoad`
- `SetAnimationBlendTime`
- `SetAnimationPose`
- `SetLastPost`
- ``vftable'`
- `operator=`
- `~AnimationSet` `V`

### `Apparatus` (Game.dll, 11)

- `Apparatus`
- `GetPauseTime` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Apparatus` `V`

### `Archive` (Engine.dll, 7)

- `Archive`
- `Close`
- `FindMatchingFiles`
- `GetFileEntry` `C`
- `GetNumFiles` `C`
- `OpenRead`
- `~Archive`

### `AreaOfInterest` (Game.dll, 13)

- `AppendDetailMapData` `V`
- `AreaOfInterest`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~AreaOfInterest` `V`

### `AreaTrigger` (Game.dll, 25)

- `AddToScene` `V`
- `AreaTrigger`
- `Detect` `V`
- `Filter` `C`
- `GetIntersection` `VC`
- `GetItems` `C`
- `GetLocalPlayer` `C`
- `GetMeshInstance` `VC`
- `GetPlayersInGame` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsDetectable` `C`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RenderEditorBounds` `V`
- `ResolveEnum_Detection` `S`
- `ShouldTrigger` `VC`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~AreaTrigger` `V`

### `AreaTriggerHookPack` (Game.dll, 6)

- `AreaTriggerHookPack`
- `AreaTriggerHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~AreaTriggerHookPack` `V`

### `Armor` (Game.dll, 31)

- `Armor`
- `AttachItem` `V`
- `ChangeArmorMesh`
- `CreateUIAttributeText` `VC`
- `DetachItem` `V`
- `GetArmorClass` `C`
- `GetGameDescription` `VC`
- `GetGenderBaseTexture` `C`
- `GetGenderBumpTexture` `C`
- `GetGenderMesh` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIGameDescription` `VC`
- `GetUIQualityDescription` `VC`
- `HasMatchingBaseTexture` `VC`
- `HasMatchingBumpTexture` `VC`
- `HasMatchingMesh` `VC`
- `Load` `V`
- `PlayBlockSound`
- `PlayImpactSound`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ResetArmorGender` `V`
- `SetArmorGender` `V`
- `UpdateTransmute` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Armor` `V`

### `ArmorJewelry` (Game.dll, 13)

- `ArmorJewelry`
- `GetHintTag` `VC`
- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIBitmapOverlay` `VC`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorJewelry` `V`

### `ArmorJewelry_Amulet` (Game.dll, 11)

- `ArmorJewelry_Amulet`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorJewelry_Amulet` `V`

### `ArmorJewelry_Bracelet` (Game.dll, 11)

- `ArmorJewelry_Bracelet`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorJewelry_Bracelet` `V`

### `ArmorJewelry_Medal` (Game.dll, 13)

- `ArmorJewelry_Medal`
- `AttachItem` `V`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorJewelry_Medal` `V`

### `ArmorJewelry_Ring` (Game.dll, 11)

- `ArmorJewelry_Ring`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorJewelry_Ring` `V`

### `ArmorMisc` (Game.dll, 11)

- `ArmorMisc`
- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorMisc` `V`

### `ArmorMisc_Clothing` (Game.dll, 15)

- `ArmorMisc_Clothing`
- `AttachItem` `V`
- `DetachItem` `V`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `SetClothing` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorMisc_Clothing` `V`

### `ArmorMisc_Vestment` (Game.dll, 15)

- `ArmorMisc_Vestment`
- `AttachItem` `V`
- `DetachItem` `V`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `SetVestment` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorMisc_Vestment` `V`

### `ArmorProtective` (Game.dll, 11)

- `ArmorProtective`
- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective` `V`

### `ArmorProtective_Chest` (Game.dll, 13)

- `ArmorProtective_Chest`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `ResetArmorGender` `V`
- `SetArmorGender` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Chest` `V`

### `ArmorProtective_Feet` (Game.dll, 11)

- `ArmorProtective_Feet`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Feet` `V`

### `ArmorProtective_Hands` (Game.dll, 11)

- `ArmorProtective_Hands`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Hands` `V`

### `ArmorProtective_Head` (Game.dll, 19)

- `ArmorProtective_Head`
- `AttachItem` `V`
- `ChangeHeadMesh`
- `DetachItem` `V`
- `GetAlternateMesh`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `ResetHeadMesh`
- `SetHeadMesh`
- `UpdateTransmute` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Head` `V`

### `ArmorProtective_Legs` (Game.dll, 11)

- `ArmorProtective_Legs`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Legs` `V`

### `ArmorProtective_Shoulders` (Game.dll, 11)

- `ArmorProtective_Shoulders`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Shoulders` `V`

### `ArmorProtective_Waist` (Game.dll, 11)

- `ArmorProtective_Waist`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ArmorProtective_Waist` `V`

### `AscendantAltar` (Game.dll, 46)

- `ActivateAltar`
- `AddSocialTarget`
- `AnimationCallback` `V`
- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `AscendantAltar`
- `DeleteSocialTarget`
- `GetGameDescription` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSocialTarget`
- `GetStaticClassInfo` `S`
- `GoActivated`
- `GoDormant`
- `GoDormantToActivated`
- `InitialUpdate` `V`
- `IsChatting`
- `IsChattingWithPlayer`
- `IsOfInterest` `VC`
- `Load` `V`
- `OccludesPathing` `VC`
- `OnConversationEnd`
- `OnDestroy` `V`
- `PlaceEffectsInWorld`
- `PlayAnimationAndFX`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetAltarState`
- `SetState`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `StartActivatedEffect`
- `StartDormantEffect`
- `StartDormantToActivatedEffect`
- `UpdateSelf` `V`
- `UpdateSocialTargetList`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~AscendantAltar` `V`

### `AscendantAltarFormula` (Game.dll, 20)

- `AscendantAltarFormula`
- `GetBossAttributes` `C`
- `GetHeaderTag`
- `GetMonsterAttributes` `C`
- `GetNonBossAttributes` `C`
- `GetRTTIClassInfo` `VC`
- `GetReagentText`
- `GetStaticClassInfo` `S`
- `Install`
- `IsBluePrintValid` `VC`
- `IsValidArtifact` `VC`
- `Load` `V`
- `LoadSwaps`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~AscendantAltarFormula` `V`

### `AsyncWorker` (Engine.dll, 6)

- `AddJob`
- `AsyncWorker`
- `CancelAll`
- `CancelJob`
- `ProcessJobQueue`
- `~AsyncWorker`

### `AttachItemConfigCmd` (Game.dll, 7)

- `AttachItemConfigCmd`
- `AttachItemConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~AttachItemConfigCmd` `V`

### `AttachItemConfigCmdPacket` (Game.dll, 8)

- `AttachItemConfigCmdPacket`
- `AttachItemConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AttachItemConfigCmdPacket` `V`

### `AttachPetAutocastConfigCmd` (Game.dll, 7)

- `AttachPetAutocastConfigCmd`
- `AttachPetAutocastConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~AttachPetAutocastConfigCmd` `V`

### `AttachPetAutocastConfigCmdPacket` (Game.dll, 8)

- `AttachPetAutocastConfigCmdPacket`
- `AttachPetAutocastConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~AttachPetAutocastConfigCmdPacket` `V`

### `AttackAction` (Game.dll, 13)

- `AnimationCallback` `V`
- `AttackAction`
- `AttackAction`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `GetSkillNumber` `C`
- `GetTargetIsValid` `C`
- `PendingActionAborted` `V`
- `QueryActionPermission` `VC`
- `ToString` `VC`
- ``vftable'`
- `~AttackAction` `V`

### `AttackPacket` (Game.dll, 8)

- `AttackPacket`
- `AttackPacket`
- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~AttackPacket` `V`

### `AttackRandoms` (Game.dll, 7)

- `AttackRandoms`
- `AttackRandoms`
- `Fill`
- ``default constructor closure'`
- ``vftable'`
- `operator=`
- `~AttackRandoms` `V`

### `AttributePak` (Game.dll, 42)

- `AttributePak`
- `AttributePak`
- `GetCharAttributes` `VC`
- `GetCharStore`
- `GetCharStoreMax`
- `GetCharStoreMin`
- `GetConversionAttributes` `VC`
- `GetDamageStore`
- `GetDamageStoreMax`
- `GetDamageStoreMin`
- `GetDefenseAttributes` `VC`
- `GetDefenseStore`
- `GetDefenseStoreMax`
- `GetDefenseStoreMin`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetRetaliationStore`
- `GetRetaliationStoreMax`
- `GetRetaliationStoreMin`
- `GetSkillAttributes` `VC`
- `GetSkillStore`
- `GetSkillStoreMax`
- `GetSkillStoreMin`
- `GetUIDisplayNextText` `VC`
- `GetUIDisplayText` `VC`
- `IsCharacterAttributePresent` `VC`
- `IsDamageTypePresent` `VC`
- `IsDefenseTypePresent` `VC`
- `IsRetaliationPresent` `VC`
- `IsRetaliationTypePresent` `VC`
- `IsSkillAttributePresent` `VC`
- `LoadFromDatabase` `V`
- `LoadFromTable` `V`
- `Merge`
- `MergeAtLevel`
- `SetDefensiveCombatRegion` `VC`
- `SetRandomGen` `V`
- ``vftable'`
- `operator=`
- `~AttributePak` `V`

### `AttributeRange` (Game.dll, 8)

- `AttributeRange`
- `AttributeRange`
- `AttributeRange`
- `CreateText` `C`
- `LoadAffix`
- `operator=`
- `operator=`
- `~AttributeRange`

### `AuraContainer` (Game.dll, 4)

- `AuraContainer`
- `AuraContainer`
- `operator=`
- `~AuraContainer`

### `AuraManager` (Game.dll, 10)

- `AddAura`
- `AuraManager`
- `AuraManager`
- `CalculateAllocatedMemory` `C`
- `Clear`
- `RemoveAura`
- `RemoveAura`
- `Update`
- `operator=`
- `~AuraManager`

### `BandariTeleportPoint` (Game.dll, 10)

- `BandariTeleportPoint`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~BandariTeleportPoint` `V`

### `BaseCounter<0>` (Engine.dll, 3)

- `BaseCounter<0>`
- `operator=`
- `operator=`

### `BaseCounter<1>` (Engine.dll, 3)

- `BaseCounter<1>`
- `operator=`
- `operator=`

### `BaseResourceManager` (Engine.dll, 20)

- `AppendResourceList` `C`
- `BaseResourceManager`
- `DecrementMemoryUsage`
- `DestroyAllResources`
- `EnablePreLoading`
- `EvictOldResources`
- `GetMemoryUsage` `C`
- `GetName` `C`
- `GetNumResources` `C`
- `IncrementMemoryUsage`
- `ListLoadedResources`
- `LoadResource` `V`
- `MaintainBudget` `V`
- `PreLoadResource`
- `SetMemoryBudget`
- `UnloadAllResources`
- `UnloadResource`
- `UnloadUnreferencedResources`
- `WriteResourceLog`
- `~BaseResourceManager` `V`

### `Billboard` (Engine.dll, 16)

- `Billboard`
- `GetIntersection` `VC`
- `GetNumRenderPasses` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetTexture` `VC`
- `InitialUpdate` `V`
- `Load` `V`
- `LogInfo` `VC`
- `RenderPass` `VC`
- `SetOffsetFromOrigin`
- `SetShader`
- `SetTexture`
- `UpdateBoundingBox` `V`
- `~Billboard` `V`

### `BindingInteractable` (Game.dll, 19)

- `BindToCharacter` `V`
- `BindingInteractable`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBindingEnabled` `VC`
- `IsCharacterRegistered` `C`
- `Load` `V`
- `RTTI_new` `S`
- `RegisterCharacter`
- `RestoreState` `V`
- `SaveState` `VC`
- `ShouldSaveState` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kDefaultBindRadius` `S`
- `~BindingInteractable` `V`

### `Bitmap` (Widget.dll, 11)

- `BitBlt` `C`
- `BitBlt` `C`
- `Bitmap`
- `Create`
- `Destroy`
- `GetHandle` `C`
- `GetXSize` `C`
- `GetYSize` `C`
- `Load`
- `SetTransparentColor`
- `~Bitmap` `V`

### `BloomSectorData` (Engine.dll, 8)

- `BloomSectorData`
- `BloomSectorData`
- `BloomSectorData`
- `Copy` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~BloomSectorData` `V`

### `BoatOverlayActivity` (Engine.dll, 7)

- `BoatOverlayActivity`
- `BoatOverlayActivity`
- `Render` `VC`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~BoatOverlayActivity`

### `BonusToClientPacket` (Game.dll, 8)

- `BonusToClientPacket`
- `BonusToClientPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~BonusToClientPacket` `V`

### `BonusToServerPacket` (Game.dll, 8)

- `BonusToServerPacket`
- `BonusToServerPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~BonusToServerPacket` `V`

### `BossSectorData` (Engine.dll, 7)

- `BossSectorData`
- `BossSectorData`
- `BossSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~BossSectorData` `V`

### `ButtonEvent` (Engine.dll, 3)

- `ButtonEvent`
- `GetText` `C`
- `~ButtonEvent` `V`

### `CDKeyPacket` (Engine.dll, 8)

- `CDKeyPacket`
- `CDKeyPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CDKeyPacket` `V`

### `CPUCoreDetector` (Engine.dll, 10)

- `CPUCoreDetector`
- `CPUCoreDetector`
- `CPUCoreDetector`
- `CpuId`
- `Detect`
- `GetCoreInfo`
- `GetNumLogicalCores`
- `operator=`
- `operator=`
- `~CPUCoreDetector`

### `Camera` (Engine.dll, 28)

- `Camera`
- `GetCameraAspect` `C`
- `GetCameraToScreenMatrix` `C`
- `GetCoords` `C`
- `GetFOV` `C`
- `GetFarPlane` `C`
- `GetFrustum` `C`
- `GetFrustum` `C`
- `GetImagePoint` `C`
- `GetMinFrustum` `C`
- `GetNearPlane` `C`
- `GetRayThroughImagePoint` `C`
- `GetSubCameraToScreenMatrix` `C`
- `GetSubFrustum` `C`
- `GetSubFrustum` `C`
- `GetType` `C`
- `Project` `C`
- `ProjectToImageSpace` `C`
- `SetCameraAspect`
- `SetCoords`
- `SetFOV`
- `SetFarPlane`
- `SetHeight`
- `SetNearPlane`
- `SetType`
- `SetWidth`
- `operator=`
- `operator=`

### `CerberusGeyserMarker` (Game.dll, 15)

- `AddToWorld` `VC`
- `CerberusGeyserMarker`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `ProcessExplosion` `V`
- `ProjectileGo` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `ShouldServerSpawn` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~CerberusGeyserMarker` `V`

### `ChallengeArea` (Game.dll, 21)

- `ApplyDebuff`
- `ApplyVisualEffects`
- `ChallengeArea`
- `ChallengeArea`
- `ChallengeArea`
- `GenerateMutators`
- `GetChestLootWeightModifiers` `C`
- `GetDebuffTime` `C`
- `GetDescriptionTag` `C`
- `GetDifficultyAdjustment` `C`
- `GetDisplayUI` `C`
- `GetLayerId` `C`
- `GetMiscDropItem`
- `GetMonsterLootWeightModifiers` `C`
- `GetNameTag` `C`
- `Load` `V`
- `SetLayerId`
- ``vftable'`
- `operator=`
- `operator=`
- `~ChallengeArea`

### `ChallengeSectorData` (Engine.dll, 8)

- `ChallengeSectorData`
- `ChallengeSectorData`
- `ChallengeSectorData`
- `Copy` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~ChallengeSectorData` `V`

### `ChaosBeam` (Engine.dll, 18)

- `ChaosBeam`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Liberate`
- `Load` `V`
- `RTTI_new` `S`
- `SetAmpScale`
- `SetEndPoints`
- `SetNumBeams`
- `SetOffset`
- `SetTimeScale`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ChaosBeam` `V`

### `CharAttribute` (Game.dll, 38)

- `AddJitter` `V`
- `CalculateAllocatedMemory` `C`
- `CharAttribute`
- `CharAttribute`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `GetRangeTag` `C`
- `GetType` `C`
- `GetValue` `C`
- `IsNotEmpty` `C`
- `Jitter`
- `LoadBaseTable` `V`
- `LoadModifierTable` `V`
- `LoadModifierTableMax` `V`
- `LoadModifierTableMin` `V`
- `LoadPrefixTable` `V`
- `LoadPrefixTableMax` `V`
- `LoadPrefixTableMin` `V`
- `LoadSuffixTable` `V`
- `LoadSuffixTableMax` `V`
- `LoadSuffixTableMin` `V`
- `MaxJitter`
- `MaxJitter` `V`
- `MergeAttribute`
- `MinJitter`
- `MinJitter` `V`
- `Scale`
- `ScaleAttribute` `V`
- `SetAttributeToLevel`
- `SetBaseValue` `V`
- `SetModifiedValue`
- `SetNextModifiedValue`
- `SetWeaponType`
- ``vftable'`
- `operator=`
- `~CharAttribute` `V`

### `CharAttributeAccumulator` (Game.dll, 15)

- `AddModifier`
- `AddMultiplier`
- `AddValue`
- `CharAttributeAccumulator`
- `CharAttributeAccumulator`
- `Clear`
- `ExecuteDefense`
- `GetCombatType` `C`
- `GetDefenseAttr`
- `GetModifier` `C`
- `GetMultiplier` `C`
- `GetValue` `C`
- `SetDefenseModify`
- `operator=`
- `~CharAttributeAccumulator`

### `CharAttributeAttackSpeed` (Game.dll, 8)

- `CharAttributeAttackSpeed`
- `CharAttributeAttackSpeed`
- `CharAttributeAttackSpeed`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeAttackSpeed` `V`

### `CharAttributeAttackSpeedMod` (Game.dll, 8)

- `CharAttributeAttackSpeedMod`
- `CharAttributeAttackSpeedMod`
- `CharAttributeAttackSpeedMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeAttackSpeedMod` `V`

### `CharAttributeMod` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `CharAttributeMod`
- `CharAttributeMod`
- `CharAttributeMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod` `V`

### `CharAttributeMod_AttackSpeed` (Game.dll, 8)

- `AddToAccumulator` `VC`
- `CharAttributeMod_AttackSpeed`
- `CharAttributeMod_AttackSpeed`
- `CharAttributeMod_AttackSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_AttackSpeed` `V`

### `CharAttributeMod_AttackSpeedCap` (Game.dll, 7)

- `CharAttributeMod_AttackSpeedCap`
- `CharAttributeMod_AttackSpeedCap`
- `CharAttributeMod_AttackSpeedCap`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_AttackSpeedCap` `V`

### `CharAttributeMod_Constitution` (Game.dll, 7)

- `CharAttributeMod_Constitution`
- `CharAttributeMod_Constitution`
- `CharAttributeMod_Constitution`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Constitution` `V`

### `CharAttributeMod_Dexterity` (Game.dll, 7)

- `CharAttributeMod_Dexterity`
- `CharAttributeMod_Dexterity`
- `CharAttributeMod_Dexterity`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Dexterity` `V`

### `CharAttributeMod_HealIncrease` (Game.dll, 7)

- `CharAttributeMod_HealIncrease`
- `CharAttributeMod_HealIncrease`
- `CharAttributeMod_HealIncrease`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_HealIncrease` `V`

### `CharAttributeMod_Intelligence` (Game.dll, 7)

- `CharAttributeMod_Intelligence`
- `CharAttributeMod_Intelligence`
- `CharAttributeMod_Intelligence`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Intelligence` `V`

### `CharAttributeMod_Life` (Game.dll, 7)

- `CharAttributeMod_Life`
- `CharAttributeMod_Life`
- `CharAttributeMod_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Life` `V`

### `CharAttributeMod_LifeMultiplier` (Game.dll, 8)

- `AddToAccumulator` `VC`
- `CharAttributeMod_LifeMultiplier`
- `CharAttributeMod_LifeMultiplier`
- `CharAttributeMod_LifeMultiplier`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_LifeMultiplier` `V`

### `CharAttributeMod_LifeRegen` (Game.dll, 7)

- `CharAttributeMod_LifeRegen`
- `CharAttributeMod_LifeRegen`
- `CharAttributeMod_LifeRegen`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_LifeRegen` `V`

### `CharAttributeMod_Mana` (Game.dll, 7)

- `CharAttributeMod_Mana`
- `CharAttributeMod_Mana`
- `CharAttributeMod_Mana`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Mana` `V`

### `CharAttributeMod_ManaLimitReserve` (Game.dll, 7)

- `CharAttributeMod_ManaLimitReserve`
- `CharAttributeMod_ManaLimitReserve`
- `CharAttributeMod_ManaLimitReserve`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_ManaLimitReserve` `V`

### `CharAttributeMod_ManaLimitReserveReduction` (Game.dll, 7)

- `CharAttributeMod_ManaLimitReserveReduction`
- `CharAttributeMod_ManaLimitReserveReduction`
- `CharAttributeMod_ManaLimitReserveReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_ManaLimitReserveReduction` `V`

### `CharAttributeMod_ManaRegen` (Game.dll, 7)

- `CharAttributeMod_ManaRegen`
- `CharAttributeMod_ManaRegen`
- `CharAttributeMod_ManaRegen`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_ManaRegen` `V`

### `CharAttributeMod_PercentHealIncrease` (Game.dll, 7)

- `CharAttributeMod_PercentHealIncrease`
- `CharAttributeMod_PercentHealIncrease`
- `CharAttributeMod_PercentHealIncrease`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_PercentHealIncrease` `V`

### `CharAttributeMod_RunSpeed` (Game.dll, 7)

- `CharAttributeMod_RunSpeed`
- `CharAttributeMod_RunSpeed`
- `CharAttributeMod_RunSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_RunSpeed` `V`

### `CharAttributeMod_RunSpeedCap` (Game.dll, 7)

- `CharAttributeMod_RunSpeedCap`
- `CharAttributeMod_RunSpeedCap`
- `CharAttributeMod_RunSpeedCap`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_RunSpeedCap` `V`

### `CharAttributeMod_SpellCastSpeed` (Game.dll, 7)

- `CharAttributeMod_SpellCastSpeed`
- `CharAttributeMod_SpellCastSpeed`
- `CharAttributeMod_SpellCastSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_SpellCastSpeed` `V`

### `CharAttributeMod_SpellCastSpeedCap` (Game.dll, 7)

- `CharAttributeMod_SpellCastSpeedCap`
- `CharAttributeMod_SpellCastSpeedCap`
- `CharAttributeMod_SpellCastSpeedCap`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_SpellCastSpeedCap` `V`

### `CharAttributeMod_Strength` (Game.dll, 7)

- `CharAttributeMod_Strength`
- `CharAttributeMod_Strength`
- `CharAttributeMod_Strength`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_Strength` `V`

### `CharAttributeMod_TotalSpeed` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `CharAttributeMod_TotalSpeed`
- `CharAttributeMod_TotalSpeed`
- `CharAttributeMod_TotalSpeed`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeMod_TotalSpeed` `V`

### `CharAttributeOtherSpeed` (Game.dll, 8)

- `CharAttributeOtherSpeed`
- `CharAttributeOtherSpeed`
- `CharAttributeOtherSpeed`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeOtherSpeed` `V`

### `CharAttributeOtherSpeedMod` (Game.dll, 8)

- `CharAttributeOtherSpeedMod`
- `CharAttributeOtherSpeedMod`
- `CharAttributeOtherSpeedMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeOtherSpeedMod` `V`

### `CharAttributePriMod` (Game.dll, 8)

- `CharAttributePriMod`
- `CharAttributePriMod`
- `CharAttributePriMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributePriMod` `V`

### `CharAttributeSecMod` (Game.dll, 8)

- `CharAttributeSecMod`
- `CharAttributeSecMod`
- `CharAttributeSecMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeSecMod` `V`

### `CharAttributeStore` (Game.dll, 22)

- `AddToAccumulator` `VC`
- `AddToStore` `V`
- `CalculateAllocatedMemory` `C`
- `CharAttributeStore`
- `CharAttributeStore`
- `Clear`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `GetAttributes`
- `GetAttributes` `C`
- `GetCostInfo` `C`
- `GetRandomGen`
- `IsTypePresent` `C`
- `MergeStore`
- `MergeStoreAtLevel`
- `ProcessText` `C`
- `ScaleAttributes` `V`
- `SetRandomGen`
- `SetWeaponType` `C`
- ``vftable'`
- `operator=`
- `~CharAttributeStore` `V`

### `CharAttributeStore_Bio` (Game.dll, 10)

- `CharAttributeStore_Bio`
- `CharAttributeStore_Bio`
- `CharAttributeStore_Bio`
- `GetBaseValue` `VC`
- `Load` `V`
- `SetBaseValue` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_Bio` `V`

### `CharAttributeStore_Equipment` (Game.dll, 8)

- `CharAttributeStore_Equipment`
- `CharAttributeStore_Equipment`
- `CharAttributeStore_Equipment`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_Equipment` `V`

### `CharAttributeStore_EquipmentPenaltyReduction` (Game.dll, 8)

- `CharAttributeStore_EquipmentPenaltyReduction`
- `CharAttributeStore_EquipmentPenaltyReduction`
- `CharAttributeStore_EquipmentPenaltyReduction`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_EquipmentPenaltyReduction` `V`

### `CharAttributeStore_Max` (Game.dll, 8)

- `CharAttributeStore_Max`
- `CharAttributeStore_Max`
- `CharAttributeStore_Max`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_Max` `V`

### `CharAttributeStore_Min` (Game.dll, 8)

- `CharAttributeStore_Min`
- `CharAttributeStore_Min`
- `CharAttributeStore_Min`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_Min` `V`

### `CharAttributeStore_PenaltyReductionMax` (Game.dll, 8)

- `CharAttributeStore_PenaltyReductionMax`
- `CharAttributeStore_PenaltyReductionMax`
- `CharAttributeStore_PenaltyReductionMax`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_PenaltyReductionMax` `V`

### `CharAttributeStore_PenaltyReductionMin` (Game.dll, 8)

- `CharAttributeStore_PenaltyReductionMin`
- `CharAttributeStore_PenaltyReductionMin`
- `CharAttributeStore_PenaltyReductionMin`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_PenaltyReductionMin` `V`

### `CharAttributeStore_Skill` (Game.dll, 8)

- `CharAttributeStore_Skill`
- `CharAttributeStore_Skill`
- `CharAttributeStore_Skill`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_Skill` `V`

### `CharAttributeStore_SkillPenalty` (Game.dll, 8)

- `CharAttributeStore_SkillPenalty`
- `CharAttributeStore_SkillPenalty`
- `CharAttributeStore_SkillPenalty`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_SkillPenalty` `V`

### `CharAttributeStore_SkillPenaltyReduction` (Game.dll, 8)

- `CharAttributeStore_SkillPenaltyReduction`
- `CharAttributeStore_SkillPenaltyReduction`
- `CharAttributeStore_SkillPenaltyReduction`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeStore_SkillPenaltyReduction` `V`

### `CharAttributeVal` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `CharAttributeVal`
- `CharAttributeVal`
- `CharAttributeVal`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal` `V`

### `CharAttributeValPri` (Game.dll, 8)

- `CharAttributeValPri`
- `CharAttributeValPri`
- `CharAttributeValPri`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeValPri` `V`

### `CharAttributeValSec` (Game.dll, 8)

- `CharAttributeValSec`
- `CharAttributeValSec`
- `CharAttributeValSec`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeValSec` `V`

### `CharAttributeValSpeed` (Game.dll, 13)

- `AddJitter` `V`
- `CharAttributeValSpeed`
- `CharAttributeValSpeed`
- `CharAttributeValSpeed`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `MaxJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeValSpeed` `V`

### `CharAttributeVal_ArmorDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_ArmorDexterityReqReduction`
- `CharAttributeVal_ArmorDexterityReqReduction`
- `CharAttributeVal_ArmorDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ArmorDexterityReqReduction` `V`

### `CharAttributeVal_ArmorIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_ArmorIntelligenceReqReduction`
- `CharAttributeVal_ArmorIntelligenceReqReduction`
- `CharAttributeVal_ArmorIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ArmorIntelligenceReqReduction` `V`

### `CharAttributeVal_ArmorStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_ArmorStrengthReqReduction`
- `CharAttributeVal_ArmorStrengthReqReduction`
- `CharAttributeVal_ArmorStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ArmorStrengthReqReduction` `V`

### `CharAttributeVal_AttackSpeed` (Game.dll, 7)

- `CharAttributeVal_AttackSpeed`
- `CharAttributeVal_AttackSpeed`
- `CharAttributeVal_AttackSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_AttackSpeed` `V`

### `CharAttributeVal_BaseAttackSpeed` (Game.dll, 23)

- `AddJitter` `V`
- `CharAttributeVal_BaseAttackSpeed`
- `CharAttributeVal_BaseAttackSpeed`
- `CharAttributeVal_BaseAttackSpeed`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `GetCostInfo` `VC`
- `LoadBaseTable` `V`
- `LoadPrefixTable` `V`
- `LoadPrefixTableMax` `V`
- `LoadPrefixTableMin` `V`
- `LoadSuffixTable` `V`
- `LoadSuffixTableMax` `V`
- `LoadSuffixTableMin` `V`
- `MaxJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_BaseAttackSpeed` `V`

### `CharAttributeVal_Constitution` (Game.dll, 7)

- `CharAttributeVal_Constitution`
- `CharAttributeVal_Constitution`
- `CharAttributeVal_Constitution`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Constitution` `V`

### `CharAttributeVal_DefensiveAbility` (Game.dll, 7)

- `CharAttributeVal_DefensiveAbility`
- `CharAttributeVal_DefensiveAbility`
- `CharAttributeVal_DefensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_DefensiveAbility` `V`

### `CharAttributeVal_DefensiveAbilityModifier` (Game.dll, 7)

- `CharAttributeVal_DefensiveAbilityModifier`
- `CharAttributeVal_DefensiveAbilityModifier`
- `CharAttributeVal_DefensiveAbilityModifier`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_DefensiveAbilityModifier` `V`

### `CharAttributeVal_DefensiveBlockRecoveryReduction` (Game.dll, 7)

- `CharAttributeVal_DefensiveBlockRecoveryReduction`
- `CharAttributeVal_DefensiveBlockRecoveryReduction`
- `CharAttributeVal_DefensiveBlockRecoveryReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_DefensiveBlockRecoveryReduction` `V`

### `CharAttributeVal_DeflectProjectile` (Game.dll, 7)

- `CharAttributeVal_DeflectProjectile`
- `CharAttributeVal_DeflectProjectile`
- `CharAttributeVal_DeflectProjectile`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_DeflectProjectile` `V`

### `CharAttributeVal_Dexterity` (Game.dll, 7)

- `CharAttributeVal_Dexterity`
- `CharAttributeVal_Dexterity`
- `CharAttributeVal_Dexterity`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Dexterity` `V`

### `CharAttributeVal_DodgePercent` (Game.dll, 7)

- `CharAttributeVal_DodgePercent`
- `CharAttributeVal_DodgePercent`
- `CharAttributeVal_DodgePercent`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_DodgePercent` `V`

### `CharAttributeVal_EnergyAbsorptionPercent` (Game.dll, 7)

- `CharAttributeVal_EnergyAbsorptionPercent`
- `CharAttributeVal_EnergyAbsorptionPercent`
- `CharAttributeVal_EnergyAbsorptionPercent`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_EnergyAbsorptionPercent` `V`

### `CharAttributeVal_GlobalReqReduction` (Game.dll, 7)

- `CharAttributeVal_GlobalReqReduction`
- `CharAttributeVal_GlobalReqReduction`
- `CharAttributeVal_GlobalReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_GlobalReqReduction` `V`

### `CharAttributeVal_HuntingDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_HuntingDexterityReqReduction`
- `CharAttributeVal_HuntingDexterityReqReduction`
- `CharAttributeVal_HuntingDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_HuntingDexterityReqReduction` `V`

### `CharAttributeVal_HuntingIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_HuntingIntelligenceReqReduction`
- `CharAttributeVal_HuntingIntelligenceReqReduction`
- `CharAttributeVal_HuntingIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_HuntingIntelligenceReqReduction` `V`

### `CharAttributeVal_HuntingStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_HuntingStrengthReqReduction`
- `CharAttributeVal_HuntingStrengthReqReduction`
- `CharAttributeVal_HuntingStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_HuntingStrengthReqReduction` `V`

### `CharAttributeVal_IncreasedExperience` (Game.dll, 7)

- `CharAttributeVal_IncreasedExperience`
- `CharAttributeVal_IncreasedExperience`
- `CharAttributeVal_IncreasedExperience`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_IncreasedExperience` `V`

### `CharAttributeVal_IncreasedGold` (Game.dll, 7)

- `CharAttributeVal_IncreasedGold`
- `CharAttributeVal_IncreasedGold`
- `CharAttributeVal_IncreasedGold`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_IncreasedGold` `V`

### `CharAttributeVal_Intelligence` (Game.dll, 7)

- `CharAttributeVal_Intelligence`
- `CharAttributeVal_Intelligence`
- `CharAttributeVal_Intelligence`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Intelligence` `V`

### `CharAttributeVal_JewelryDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_JewelryDexterityReqReduction`
- `CharAttributeVal_JewelryDexterityReqReduction`
- `CharAttributeVal_JewelryDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_JewelryDexterityReqReduction` `V`

### `CharAttributeVal_JewelryIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_JewelryIntelligenceReqReduction`
- `CharAttributeVal_JewelryIntelligenceReqReduction`
- `CharAttributeVal_JewelryIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_JewelryIntelligenceReqReduction` `V`

### `CharAttributeVal_JewelryStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_JewelryStrengthReqReduction`
- `CharAttributeVal_JewelryStrengthReqReduction`
- `CharAttributeVal_JewelryStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_JewelryStrengthReqReduction` `V`

### `CharAttributeVal_LevelReqReduction` (Game.dll, 11)

- `AddJitter` `V`
- `CharAttributeVal_LevelReqReduction`
- `CharAttributeVal_LevelReqReduction`
- `CharAttributeVal_LevelReqReduction`
- `MaxJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_LevelReqReduction` `V`

### `CharAttributeVal_Life` (Game.dll, 7)

- `CharAttributeVal_Life`
- `CharAttributeVal_Life`
- `CharAttributeVal_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Life` `V`

### `CharAttributeVal_LifeRegen` (Game.dll, 7)

- `CharAttributeVal_LifeRegen`
- `CharAttributeVal_LifeRegen`
- `CharAttributeVal_LifeRegen`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_LifeRegen` `V`

### `CharAttributeVal_LightRadius` (Game.dll, 7)

- `CharAttributeVal_LightRadius`
- `CharAttributeVal_LightRadius`
- `CharAttributeVal_LightRadius`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_LightRadius` `V`

### `CharAttributeVal_Mana` (Game.dll, 7)

- `CharAttributeVal_Mana`
- `CharAttributeVal_Mana`
- `CharAttributeVal_Mana`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Mana` `V`

### `CharAttributeVal_ManaLimitReserve` (Game.dll, 8)

- `CharAttributeVal_ManaLimitReserve`
- `CharAttributeVal_ManaLimitReserve`
- `CharAttributeVal_ManaLimitReserve`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ManaLimitReserve` `V`

### `CharAttributeVal_ManaLimitReserveReduction` (Game.dll, 7)

- `CharAttributeVal_ManaLimitReserveReduction`
- `CharAttributeVal_ManaLimitReserveReduction`
- `CharAttributeVal_ManaLimitReserveReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ManaLimitReserveReduction` `V`

### `CharAttributeVal_ManaRegen` (Game.dll, 7)

- `CharAttributeVal_ManaRegen`
- `CharAttributeVal_ManaRegen`
- `CharAttributeVal_ManaRegen`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ManaRegen` `V`

### `CharAttributeVal_MeleeDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_MeleeDexterityReqReduction`
- `CharAttributeVal_MeleeDexterityReqReduction`
- `CharAttributeVal_MeleeDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_MeleeDexterityReqReduction` `V`

### `CharAttributeVal_MeleeIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_MeleeIntelligenceReqReduction`
- `CharAttributeVal_MeleeIntelligenceReqReduction`
- `CharAttributeVal_MeleeIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_MeleeIntelligenceReqReduction` `V`

### `CharAttributeVal_MeleeStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_MeleeStrengthReqReduction`
- `CharAttributeVal_MeleeStrengthReqReduction`
- `CharAttributeVal_MeleeStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_MeleeStrengthReqReduction` `V`

### `CharAttributeVal_OffensiveAbility` (Game.dll, 7)

- `CharAttributeVal_OffensiveAbility`
- `CharAttributeVal_OffensiveAbility`
- `CharAttributeVal_OffensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_OffensiveAbility` `V`

### `CharAttributeVal_OffensiveAbilityModifier` (Game.dll, 7)

- `CharAttributeVal_OffensiveAbilityModifier`
- `CharAttributeVal_OffensiveAbilityModifier`
- `CharAttributeVal_OffensiveAbilityModifier`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_OffensiveAbilityModifier` `V`

### `CharAttributeVal_RunSpeed` (Game.dll, 7)

- `CharAttributeVal_RunSpeed`
- `CharAttributeVal_RunSpeed`
- `CharAttributeVal_RunSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_RunSpeed` `V`

### `CharAttributeVal_ShieldDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_ShieldDexterityReqReduction`
- `CharAttributeVal_ShieldDexterityReqReduction`
- `CharAttributeVal_ShieldDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ShieldDexterityReqReduction` `V`

### `CharAttributeVal_ShieldIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_ShieldIntelligenceReqReduction`
- `CharAttributeVal_ShieldIntelligenceReqReduction`
- `CharAttributeVal_ShieldIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ShieldIntelligenceReqReduction` `V`

### `CharAttributeVal_ShieldStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_ShieldStrengthReqReduction`
- `CharAttributeVal_ShieldStrengthReqReduction`
- `CharAttributeVal_ShieldStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_ShieldStrengthReqReduction` `V`

### `CharAttributeVal_SpellCastSpeed` (Game.dll, 7)

- `CharAttributeVal_SpellCastSpeed`
- `CharAttributeVal_SpellCastSpeed`
- `CharAttributeVal_SpellCastSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_SpellCastSpeed` `V`

### `CharAttributeVal_StaffDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_StaffDexterityReqReduction`
- `CharAttributeVal_StaffDexterityReqReduction`
- `CharAttributeVal_StaffDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_StaffDexterityReqReduction` `V`

### `CharAttributeVal_StaffIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_StaffIntelligenceReqReduction`
- `CharAttributeVal_StaffIntelligenceReqReduction`
- `CharAttributeVal_StaffIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_StaffIntelligenceReqReduction` `V`

### `CharAttributeVal_StaffStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_StaffStrengthReqReduction`
- `CharAttributeVal_StaffStrengthReqReduction`
- `CharAttributeVal_StaffStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_StaffStrengthReqReduction` `V`

### `CharAttributeVal_Strength` (Game.dll, 7)

- `CharAttributeVal_Strength`
- `CharAttributeVal_Strength`
- `CharAttributeVal_Strength`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Strength` `V`

### `CharAttributeVal_Weapon2HDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_Weapon2HDexterityReqReduction`
- `CharAttributeVal_Weapon2HDexterityReqReduction`
- `CharAttributeVal_Weapon2HDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Weapon2HDexterityReqReduction` `V`

### `CharAttributeVal_Weapon2HIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_Weapon2HIntelligenceReqReduction`
- `CharAttributeVal_Weapon2HIntelligenceReqReduction`
- `CharAttributeVal_Weapon2HIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Weapon2HIntelligenceReqReduction` `V`

### `CharAttributeVal_Weapon2HStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_Weapon2HStrengthReqReduction`
- `CharAttributeVal_Weapon2HStrengthReqReduction`
- `CharAttributeVal_Weapon2HStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_Weapon2HStrengthReqReduction` `V`

### `CharAttributeVal_WeaponDexterityReqReduction` (Game.dll, 7)

- `CharAttributeVal_WeaponDexterityReqReduction`
- `CharAttributeVal_WeaponDexterityReqReduction`
- `CharAttributeVal_WeaponDexterityReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_WeaponDexterityReqReduction` `V`

### `CharAttributeVal_WeaponIntelligenceReqReduction` (Game.dll, 7)

- `CharAttributeVal_WeaponIntelligenceReqReduction`
- `CharAttributeVal_WeaponIntelligenceReqReduction`
- `CharAttributeVal_WeaponIntelligenceReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_WeaponIntelligenceReqReduction` `V`

### `CharAttributeVal_WeaponStrengthReqReduction` (Game.dll, 7)

- `CharAttributeVal_WeaponStrengthReqReduction`
- `CharAttributeVal_WeaponStrengthReqReduction`
- `CharAttributeVal_WeaponStrengthReqReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~CharAttributeVal_WeaponStrengthReqReduction` `V`

### `CharFxPak` (Game.dll, 24)

- `CalculateAllocatedMemory` `C`
- `CharFxPak`
- `CharFxPak`
- `DisableSound`
- `Enable` `V`
- `GetSkillId` `C`
- `InitializeFxPak` `V`
- `LoadFromDatabase` `V`
- `LoadMesh`
- `LoadParticle`
- `Pause` `V`
- `PreLoad` `V`
- `RemoveMesh`
- `RemoveParticles`
- `SetDebuff` `V`
- `SetForceAddToScene` `V`
- `SetParent`
- `SetSkillId`
- `Start` `V`
- `Stop` `V`
- `UnLoad` `V`
- `UpdateVisibility` `V`
- ``vftable'`
- `~CharFxPak` `V`

### `CharPickUpConfigCmd` (Game.dll, 7)

- `CharPickUpConfigCmd`
- `CharPickUpConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~CharPickUpConfigCmd` `V`

### `Character` (Game.dll, 714)

- `ActivateSkill`
- `AddAffinity`
- `AddAttachedEntitiesToScene` `V`
- `AddAura` `V`
- `AddCombatFilter`
- `AddDevotionPoints`
- `AddEquipmentProp`
- `AddItemSkillModifiers`
- `AddItemSkills`
- `AddItemToSet`
- `AddLifeSlow`
- `AddManaSlow`
- `AddMeshEffect` `V`
- `AddModifierPoints`
- `AddMoney`
- `AddParticleEffect` `V`
- `AddSkillPoints`
- `AddToModifierPoints`
- `AddToScene` `V`
- `AddToSkillAugment`
- `AddToSkillLevel`
- `AddToSkillPoints`
- `AddTotalDevotionPoints`
- `AddTribute`
- `AllocateAllAnimSets` `VC`
- `AllocateAnimSets`
- `AllowDualWieldWeapons`
- `AllyAttacked` `V`
- `AlreadyThere` `C`
- `AlwaysWarps` `C`
- `AnimationCallback` `V`
- `ApplyDamage`
- `AreMessagesVisible` `C`
- `AttachItem`
- `AttachItemAction` `V`
- `AttachPetAutocast`
- `AttackTarget` `V`
- `BeginFreeze`
- `BeginImmobilize`
- `BeginKnockdown`
- `BeginPetrify`
- `BeginSleep`
- `BeginStun`
- `BeginTakeHit`
- `BeginTrap`
- `BuffTarget`
- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `CalculateStun` `V`
- `CanInterruptMovement` `C`
- `CanMoveTo` `VC`
- `CanPlayAnimation` `C`
- `CanWalk` `C`
- `CancelSkillAction`
- `CapAbsoluteRunSpeed` `C`
- `CapAttackSpeed` `VC`
- `CapRunSpeed` `VC`
- `CapSpellCastSpeed` `VC`
- `CausesAnger` `VC`
- `Character`
- `CharacterHasDied` `V`
- `CharacterIsDying` `V`
- `CharacterStop` `V`
- `ChatCommandUseSkill`
- `CheckForcedUpdateLoadSphere` `VC`
- `ClearAllowedSkillTypes`
- `ClearDamageFxs`
- `ClearPointOfInterest`
- `CollisionCallback` `V`
- `CombatAddCooldownDamage` `V`
- `CombatAddLifeLeechDamage` `V`
- `CombatCausedHitReaction` `V`
- `CombatExertInfluenceConfusion` `V`
- `CombatExertInfluenceFear` `V`
- `CombatExertInfluenceTaunt` `V`
- `CompleteInventoryRelics`
- `ContributeElementalResistanceReductionAbsolute` `C`
- `ContributeElementalResistanceReductionPercent` `C`
- `ContributeGameBalanceCharAttributes` `VC`
- `ContributeGameBalanceDefenseAttributes` `VC`
- `ContributeGameBalanceOffensiveDamageAttributes` `VC`
- `ContributeGameBalanceOffensiveModifierAttributes` `VC`
- `ContributeGameBalanceRetaliationAttributes` `VC`
- `ContributeGameBalanceRetaliationModifierAttributes` `VC`
- `ContributeGameBalanceSkillAttributes` `VC`
- `ContributeMiscCharAttributes` `VC`
- `ContributeMiscConversionAttributes` `VC`
- `ContributeMiscDefenseAttributes` `VC`
- `ContributeMiscOffensiveDamageAttributes` `VC`
- `ContributeMiscOffensiveModifierAttributes` `VC`
- `ContributeMiscRetaliationAttributes` `VC`
- `ContributeMiscRetaliationModifierAttributes` `VC`
- `ContributeMiscSkillAttributes` `VC`
- `ContributeMutatorCharAttributes` `VC`
- `ContributeMutatorDefenseAttributes` `VC`
- `ContributeMutatorOffensiveDamageAttributes` `VC`
- `ContributeMutatorOffensiveModifierAttributes` `VC`
- `ContributeMutatorRetaliationAttributes` `VC`
- `ContributeMutatorRetaliationModifierAttributes` `VC`
- `ContributeMutatorSkillAttributes` `VC`
- `ContributePhysicalResistanceReductionAbsolute` `C`
- `ContributePhysicalResistanceReductionPercent` `C`
- `ContributeRacialBonusDamage` `C`
- `ContributeRacialBonusDefense` `C`
- `ContributeTotalResistanceReductionAbsolute` `C`
- `ContributeTotalResistanceReductionPercent` `C`
- `ControllerAddSkillBuff`
- `ConvertFxBuffName` `S`
- `CreateFxPak` `V`
- `CreateItemFromLootTable`
- `CreateProjectile`
- `CreateRemoveSkillBuff`
- `CreateUINextSummaryText` `VC`
- `CreateUISummaryText` `VC`
- `CrowdAgentCreated` `V`
- `CrowdAgentDepenetrate` `V`
- `CrowdAgentDestroyed` `V`
- `CrowdAgentError` `V`
- `CrowdAgentMoved` `V`
- `CrowdAgentReachedGoal` `V`
- `CrowdAgentStopped` `V`
- `CrowdAgentUpdate` `V`
- `DeactivateSkill`
- `DeathCanFinish` `VC`
- `DebufTarget`
- `DebugIncrementMasteriesAllowed`
- `DebugRender` `V`
- `DebugRenderAttachpoints`
- `DebugRenderPathing`
- `DecModifierPoints`
- `DecrementBaseDexterity`
- `DecrementBaseIntelligence`
- `DecrementBaseLife`
- `DecrementBaseMana`
- `DecrementBaseStrength`
- `DeleteOnEnteringUnloadedLevel` `VC`
- `DeleteProjectile`
- `DesignerCalculateCriticalChance` `C`
- `DesignerCalculateDefensiveAbility`
- `DesignerCalculateMagicalDamage` `C`
- `DesignerCalculateMagicalDurationDamage` `C`
- `DesignerCalculateMeleeBlockChance` `C`
- `DesignerCalculateOffensiveAbility`
- `DesignerCalculatePhysicalDamage` `C`
- `DesignerCalculatePhysicalDamageBonus` `C`
- `DesignerCalculatePhysicalDamageDefense` `C`
- `DesignerCalculatePhysicalDamagePercentage` `C`
- `DesignerCalculatePhysicalDurationDamage` `C`
- `DesignerCalculatePierceDamage` `C`
- `DesignerCalculateProbabilityToHit` `C`
- `DesignerCalculateShieldBlockDamageReduction` `C`
- `DestroyMe`
- `DetachItemAction` `V`
- `DetachSpawnEffect`
- `DisableMovement`
- `DisallowsMovement` `C`
- `DisarmAbsEffects` `V`
- `DispelDamageOverTime` `V`
- `DispelSkillBuffs` `V`
- `DispelSkillDeBuffs` `V`
- `DissolveAttachments`
- `DoDebugRender` `VC`
- `DoDistressCall` `V`
- `DumpPlayStats` `V`
- `EnableSpawnAnimation` `V`
- `EndFreeze`
- `EndImmobilize`
- `EndKnockdown`
- `EndPetrify`
- `EndSleep`
- `EndStun`
- `EndTakeHit`
- `EndTrap`
- `EvadeSkill`
- `ExecuteImmobilize` `V`
- `ExecuteKnockdown` `V`
- `ExecuteSleep` `V`
- `ExecuteStun` `V`
- `ExecuteTakeHit` `V`
- `ExecuteTrap` `V`
- `FactionLevelDown`
- `FactionLevelUp`
- `FindItemSkillId` `C`
- `FindSkillId` `C`
- `FireAbsEffect` `V`
- `ForceSpeedUpdate`
- `ForceUpdateResources` `V`
- `GetActionHandler`
- `GetActionState` `VC`
- `GetActionStateAsText` `C`
- `GetActionStateAsText` `S`
- `GetActiveStatusSkills`
- `GetActorToSpawnForMyBones` `C`
- `GetAffinity` `C`
- `GetAllDefenseAttributes` `C`
- `GetAllowedOffNavmesh` `C`
- `GetAmbientHighlight` `VC`
- `GetAngerMultiplier` `C`
- `GetAnimationSet` `C`
- `GetAttachedItems` `C`
- `GetAttackSound` `C`
- `GetAttackSpeed`
- `GetAttackerId` `C`
- `GetBaseCharAttribute` `C`
- `GetBaseCharAttributes` `C`
- `GetChanceToHit` `V`
- `GetChangeOfEquipmentNotification`
- `GetCharLevel` `C`
- `GetCharLevelGapFixer` `VC`
- `GetCharacterBio`
- `GetCharacterBio` `C`
- `GetCharacterHooks`
- `GetChatCoords` `VC`
- `GetChatterPack` `C`
- `GetCollisionBox` `VC`
- `GetCollisionRepresentation` `VC`
- `GetCombatManager`
- `GetCombatManager` `C`
- `GetCombatRegionChance` `C`
- `GetConstitutionLimit` `C`
- `GetControllerId` `C`
- `GetConversation` `C`
- `GetConversationPartnerLocation`
- `GetConvertLevel` `VC`
- `GetCriticalHitSound` `VC`
- `GetCurrentAnimation` `VC`
- `GetCurrentAttackTarget`
- `GetCurrentConstitution` `C`
- `GetCurrentLife` `C`
- `GetCurrentLifeInt` `C`
- `GetCurrentMana` `C`
- `GetCurrentMoney` `C`
- `GetCurrentTribute` `C`
- `GetDamageMultiplier` `C`
- `GetDeathEffect` `C`
- `GetDeathManager`
- `GetDeathSpawner` `C`
- `GetDefaultAnimationSet` `C`
- `GetDefenseAttributeCap` `VC`
- `GetDeleteBehavior` `C`
- `GetDevotionPoints` `C`
- `GetDexterityLifeIncrement` `C`
- `GetDismissBehavior` `C`
- `GetDissolveColorBlue` `C`
- `GetDissolveColorGreen` `C`
- `GetDissolveColorRed` `C`
- `GetDissolveEffect` `C`
- `GetDissolveLight` `C`
- `GetDissolveTexture` `C`
- `GetDissolveTime`
- `GetDistressCallGroup` `C`
- `GetDurationDamageMgr`
- `GetEffectIndex` `C`
- `GetElementalDamageReductionPercent` `C`
- `GetEquipArmorDamageAttributes` `C`
- `GetEquipCharAttributes` `C`
- `GetEquipConversionAttributes` `C`
- `GetEquipOffensiveModifierAttributes` `C`
- `GetEquipRetaliationAttributes` `C`
- `GetEquipRetaliationModifierAttributes` `C`
- `GetEquipSkillAttributes` `C`
- `GetExperiencePoints` `C`
- `GetExperienceReward` `V`
- `GetExtents` `VC`
- `GetFaction` `C`
- `GetFactionPack`
- `GetFactionPack` `C`
- `GetFactionValue`
- `GetFootCoords` `V`
- `GetFumbleDamage` `C`
- `GetFurthestMoveToPoint`
- `GetGarmentManager`
- `GetGender` `VC`
- `GetGibEffectName` `C`
- `GetGibSoundName` `C`
- `GetGibThreshold` `C`
- `GetGoldGenerator` `C`
- `GetGoldGeneratorChance` `C`
- `GetHandStateAnimation`
- `GetIntelligenceLifeIncrement` `C`
- `GetInventory`
- `GetInventoryItems` `C`
- `GetInventoryReplica` `C`
- `GetItemCount` `C`
- `GetItemCountInStashes` `VC`
- `GetItemSkillCache` `C`
- `GetItemSkillList` `C`
- `GetItemsInSet` `C`
- `GetLastAttackTime` `C`
- `GetLastHitFrame` `C`
- `GetLastLevelExperience` `C`
- `GetLeader` `VC`
- `GetLifeLimit` `C`
- `GetLifePerDexterity` `C`
- `GetLifePerIntelligence` `C`
- `GetLifePerStrength` `C`
- `GetLifeRegenValue` `C`
- `GetLifeState` `VC`
- `GetLifeStateAsText` `VC`
- `GetLivingCharacters`
- `GetLowerHealthDisplayPercentage` `C`
- `GetManaLimit` `C`
- `GetManaPerIntel` `C`
- `GetManaRegenValue` `C`
- `GetMaxDevotionPoints` `C`
- `GetMaxRotationSpeed` `C`
- `GetMeshInstance` `VC`
- `GetMinRotationSpeed` `C`
- `GetModifierPoints` `C`
- `GetMoveToDistance` `C`
- `GetMoveToPoint` `C`
- `GetMovementTarget` `C`
- `GetNextLevelExperience` `C`
- `GetNumberOfItemsInSet` `C`
- `GetOffensiveReduction` `C`
- `GetOriginalFaction` `VC`
- `GetOriginalFactionPack` `VC`
- `GetOriginalMeshInstance` `C`
- `GetOverKilled` `C`
- `GetOverrideAnimationName` `C`
- `GetPathGenerationParams`
- `GetPathMass` `C`
- `GetPathPosition` `C`
- `GetPathSlowdownLength` `VC`
- `GetPathTarget` `C`
- `GetPetPen`
- `GetPetPen` `C`
- `GetPhysicalDamageReductionPercent` `C`
- `GetPhysicsTimeLimit` `C`
- `GetPlayStats`
- `GetPlayStats` `C`
- `GetPointAwayFromGoal` `C`
- `GetPointsSpent` `C`
- `GetPortraitName` `C`
- `GetPotentialConstitution` `C`
- `GetPotentialLife` `C`
- `GetPotentialMana` `C`
- `GetProjectileFumbleDamage` `C`
- `GetQuestPetPen`
- `GetRTTIClassInfo` `VC`
- `GetRace` `VC`
- `GetRaceText` `VC`
- `GetRagDollBehaviorOverride` `C`
- `GetRagDollSpeedOverride` `C`
- `GetRagdollData` `C`
- `GetReflectCap` `VC`
- `GetRegionBoundingSphere` `C`
- `GetRemainingAnimationTime` `C`
- `GetReserveMana` `C`
- `GetRotateTowardsPoint`
- `GetRunSpeed`
- `GetShouldRenderAcrossPortals` `VC`
- `GetShowAngerLevels` `C`
- `GetSize` `VC`
- `GetSkillCharAttributes` `C`
- `GetSkillId` `C`
- `GetSkillList` `C`
- `GetSkillManager`
- `GetSkillManager` `C`
- `GetSkillMasteries` `C`
- `GetSkillMasteriesActive` `C`
- `GetSkillMasteriesAllowed` `C`
- `GetSkillPoints` `C`
- `GetSkillReferenceNumber` `C`
- `GetSleepAggressionFalloffRate` `VC`
- `GetSpawnAnimationEnabled` `VC`
- `GetSpawnPoint` `C`
- `GetSpeed` `C`
- `GetSpellCastSpeed`
- `GetStaticClassInfo` `S`
- `GetStraightMoveToPoint`
- `GetStrengthLifeIncrement` `C`
- `GetSubSkillsList` `C`
- `GetSwipeSound` `C`
- `GetTargetDistance` `S`
- `GetTotalAllLevelAugment`
- `GetTotalCharAttribute` `C`
- `GetTotalCharModifier` `C`
- `GetTotalDamageReductionAbsolute` `C`
- `GetTotalDamageReductionPercent` `C`
- `GetTotalDevotionPoints` `C`
- `GetTotalMasteryLevelAugment`
- `GetUISkillList` `C`
- `GetUITempSkillList` `C`
- `GetUnarmedWeaponTrailLH` `C`
- `GetUnarmedWeaponTrailRH` `C`
- `GetUpperHealthDisplayPercentage` `C`
- `GetUsableRange` `C`
- `GetVisibleFaction` `C`
- `GetWalkAnimationType` `C`
- `GetWalkDistance` `C`
- `GetWalkSpeed` `C`
- `GetWeaponAnimation`
- `GetWeaponScale` `C`
- `GiveArtifactToCharacter` `V`
- `GiveAscendedItemToCharacter` `V`
- `GiveDismantledBonusItemToCharacter` `V`
- `GiveDismantledItemToCharacter` `V`
- `GiveExperience` `VC`
- `GiveItemToCharacter` `V`
- `GiveRecoveredItemToCharacter` `V`
- `GiveRerollItemToCharacter` `V`
- `GiveSetItemToCharacter` `V`
- `GiveTinkeredItemToCharacter` `V`
- `GoIdle`
- `HandleSkillAnimationCallback`
- `HasItem` `C`
- `HasItem` `C`
- `HasItem` `C`
- `HasModifierPointsInUse` `C`
- `IgnoreDissolveDelay` `C`
- `IgnoreWhenPathing` `C`
- `ImDead` `V`
- `ImDying` `V`
- `InChallengeArea` `C`
- `InEndlessDungeon` `C`
- `IncModifierPoints`
- `IncrementBaseAttribute`
- `IncrementBaseDexterity`
- `IncrementBaseIntelligence`
- `IncrementBaseLife`
- `IncrementBaseMana`
- `IncrementBaseMana`
- `IncrementBaseStrength`
- `IncrementCharLevel`
- `InitialAnimation` `V`
- `InitialUpdate` `V`
- `IsAlive` `VC`
- `IsAttackMoving` `C`
- `IsCharacter` `V`
- `IsControllingCharacter` `C`
- `IsEvading` `C`
- `IsFrozenOrPetrified` `C`
- `IsGod` `C`
- `IsHiding` `VC`
- `IsImmobilized` `VC`
- `IsImmuneToPlague` `C`
- `IsInvincible` `VC`
- `IsInvincibleInDbr` `C`
- `IsJumping` `C`
- `IsKnockedDown` `VC`
- `IsMaxLevel` `C`
- `IsMenuCharacter` `C`
- `IsMovementDisabled` `C`
- `IsMoving` `C`
- `IsOfInterest` `VC`
- `IsOmnipotent` `C`
- `IsPathing` `C`
- `IsPoisoned` `C`
- `IsQuestPet` `C`
- `IsRegisteredForForcedUpdates` `C`
- `IsSkillBuffActive` `C`
- `IsSkillToggled` `C`
- `IsTargetable` `VC`
- `IsTargetableInDbr` `VC`
- `IsTeleporting` `C`
- `IsUnarmed`
- `IsUnderAttack` `C`
- `JoinMe` `V`
- `JumpToUseSkill`
- `JustSpawnedWithAnimation` `VC`
- `Load` `V`
- `LoadAnimation` `V`
- `LoadAnimationSound`
- `LoadAnimationSound`
- `LoadAnimations`
- `LoadLoot`
- `LoadLootFromSeed`
- `LoadLootFromStruct`
- `LoadSoundPak`
- `LogInventory` `VC`
- `LostSlot`
- `MoveTo`
- `NearTarget` `V`
- `NotifyControllerItemRemovedFromInventory` `V`
- `OccludesPathing` `VC`
- `OnAddToLevel` `V`
- `OnAddToWorld` `V`
- `OnAttack` `V`
- `OnAttacked` `V`
- `OnCallbackDie` `V`
- `OnCallbackSkillScript` `V`
- `OnDestroy` `V`
- `OnDurationDamage` `V`
- `OnKilledPlayer` `V`
- `OnMoveInLevel` `V`
- `OnPlayNetSound`
- `OnRemoveFromWorld` `V`
- `OnSkillInterrupted` `V`
- `OnTeleported` `V`
- `OneShotParticleEffect`
- `PhysicsPost` `V`
- `PhysicsResponse` `V`
- `PhysicsSetup` `V`
- `PhysicsSync` `V`
- `PhysicsUpdate` `V`
- `PlayAnimationSound`
- `PlayArmSwipeSound` `C`
- `PlayAttackSound` `C`
- `PlayBodyFallSound`
- `PlayComboChargeRemainFX`
- `PlayCooldownResetFx`
- `PlayDeathSound`
- `PlayImpactSound` `V`
- `PlayLoopingRunningSound`
- `PlayLootSound`
- `PlayLowHealthSoundIfNeeded` `V`
- `PlayNetSound`
- `PlayOverridableSound2d` `C`
- `PlayOverridableSound3d` `C`
- `PlaySpecialAttackSound1`
- `PlaySpecialAttackSound2`
- `PlaySpecialAttackSound3`
- `PlaySpecialAttackSound4`
- `PlayStunSound`
- `PlayVoxSound`
- `PostPetSpawn` `V`
- `PreAnimationUpdate` `V`
- `PreLoad` `V`
- `PrimeAbsEffect` `V`
- `ProjectileCollisionCallback`
- `ProjectileNotification`
- `QuestCommandMove`
- `QuestCommandOrient`
- `QuestCommandPlayAnimation`
- `QuestCommandUseSkill`
- `QuestCommandWalk`
- `RTTI_new` `S`
- `ReceiveExperience`
- `RegisterCombatTextCrit` `V`
- `RegisterCombatTextHit` `V`
- `RegisterNetSound`
- `ReleaseAttackSlot`
- `ReleaseDefenseSlot`
- `ReleaseImmobilize` `V`
- `ReleaseKnockdown` `V`
- `ReleaseSleep` `V`
- `ReleaseStun` `V`
- `ReleaseTakeHit` `V`
- `ReleaseTrap` `V`
- `ReloadCharacterBio`
- `RemoveAllSkillLevelAugment`
- `RemoveAura` `V`
- `RemoveAura` `V`
- `RemoveCombatFilter`
- `RemoveDevotionPoints`
- `RemoveEquipmentProp`
- `RemoveItemFromSet`
- `RemoveItemSkillModifiers`
- `RemoveItemSkills`
- `RemoveMasteryLevelAugment`
- `RemoveMeshEffect` `V`
- `RemoveParticleEffect` `V`
- `RemoveSkillLevelAugment`
- `RemoveTotalDevotionPoints`
- `RequestAttack`
- `RequestAttackSlot`
- `RequestDefenseSlot`
- `RequestMove`
- `ResetAbsEffects` `V`
- `ResetAffinities`
- `ResetFactions`
- `ResetModifierPoints`
- `ResetMovementModifier`
- `RestoreLifeState`
- `RestoreState` `V`
- `RotateTowards`
- `RotateTowardsPoint`
- `RotateTowardsTarget`
- `SaveState` `VC`
- `SendQuestAnimationCompletedEvent` `C`
- `SendQuestMoveCompletedEvent` `C`
- `SendSkillAugmentUpdate`
- `SetActionHandler`
- `SetActionState` `V`
- `SetAllSkillLevelAugment`
- `SetAllowedSkillTypes`
- `SetAlwaysWarp`
- `SetAnimation` `V`
- `SetAnimationData`
- `SetBreastSupport`
- `SetCausesAnger`
- `SetCharLevel`
- `SetCharacterBuffFx`
- `SetCurrentAttackTarget`
- `SetCurrentMana`
- `SetDismissBehavior`
- `SetExtentsLarge`
- `SetExtentsSmall`
- `SetFaction`
- `SetFactionPack`
- `SetFactionValue`
- `SetForcedUpdates`
- `SetGod`
- `SetHideEquipment`
- `SetIgnoreWhenPathing`
- `SetImmuneToPlagueTime`
- `SetInEndlessDungeon`
- `SetInitialStateInfo`
- `SetInventoryReplica`
- `SetInvincible`
- `SetInvisible`
- `SetIsControllingCharacter` `V`
- `SetLastAttackerId` `V`
- `SetLevelLimits`
- `SetLifeState` `V`
- `SetLookAtCoords`
- `SetMasteryLevelAugment`
- `SetMenuCharacter`
- `SetObserverCheatMode`
- `SetObserverMode`
- `SetOmnipotent`
- `SetOverKilled`
- `SetOverrideAnimations`
- `SetOverrideAnimations`
- `SetOverrideFootsteps`
- `SetOverrideMesh`
- `SetOverrideSounds`
- `SetParticleVisibility` `V`
- `SetPathMass`
- `SetPathPosition`
- `SetPathPositionNotCoords`
- `SetPathingPushThru`
- `SetPhysicsSimulation` `V`
- `SetPointOfInterest`
- `SetQuestPet`
- `SetRagdollData`
- `SetRotationSpeedMultiplier`
- `SetSelectivePathingPushThru`
- `SetSkillLevelAugment`
- `SetSpawnPoint`
- `SetSpineCurve` `V`
- `SetSuppressEvents`
- `SetTargetable`
- `SetTeleporting`
- `SetUnarmedWeaponTrail`
- `SetWeaponEnchantment`
- `ShouldDoLateCrumple` `C`
- `ShouldDoRagDoll` `C`
- `ShouldLoadLoot` `VC`
- `ShouldRegisterCallbackToDie`
- `ShouldRenderForScene` `VC`
- `ShouldRotateWhenChatting` `C`
- `ShouldServerSpawn` `VC`
- `ShouldSuppressDeathSpawner` `C`
- `ShouldSuppressDyingSkill` `C`
- `ShouldSuppressLoot` `C`
- `ShouldUseDistressCall` `VC`
- `ShouldWaitForCallbackToSpawn`
- `ShowAngerLevels`
- `ShowErrorMessages`
- `SkillActiveStateUpdateCommand`
- `SkillAugmentUpdateCommand`
- `SkillComboChargeUpdateCommand`
- `SkillSpawnObject`
- `SkillStateUpdateCommand`
- `SkillTargetResult`
- `SkillWarmUp`
- `SpendComboCharges`
- `StartDamageEffect` `V`
- `StartInvoluntaryEffect`
- `StartMeshEffect` `V`
- `StartParticleEffect` `V`
- `StartPrespawnEffect`
- `StartRespawn` `V`
- `StartSkill`
- `StartSpawnEffect`
- `StopAttachedEffects`
- `StopCharacterBuffFx`
- `StopDamageEffect` `V`
- `StopInvoluntaryEffect`
- `StopLoopingRunningSound`
- `StopMeshEffect` `V`
- `StopMoving`
- `StopParticleEffect` `V`
- `StopRespawn` `V`
- `SubtractAffinity`
- `SubtractDevotionPoint`
- `SubtractLife`
- `SubtractMana`
- `SubtractMoney`
- `SubtractSkillPoint`
- `SubtractTribute`
- `SuppressDeathSpawner`
- `SuppressDyingSkill`
- `SuppressLoot`
- `TakeAttack` `V`
- `TakeBonus` `V`
- `TakeItemFromCharacter`
- `TakeItemFromCharacter`
- `TakeItemFromStashes` `V`
- `TakeRandomItemFromCharacter`
- `TeleportToLocation` `V`
- `TouchedByPlayer` `V`
- `TweakPose` `V`
- `TweakPose_Breasts`
- `TweakPose_Head`
- `TweakPose_Spine`
- `UnJoinLeader` `V`
- `UnderAttack` `V`
- `UpdateIdleAnimation` `V`
- `UpdatePath`
- `UpdatePathPosition`
- `UpdatePunctuation` `V`
- `UpdateSelf` `V`
- `Update_TweakPose`
- `UseInventoryItem`
- `UseItemOn`
- `WalkTo` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Character` `V`

### `CharacterAction` (Engine.dll, 6)

- `CharacterAction`
- `CharacterAction`
- `QueryActionPermission` `VC`
- ``vftable'`
- `operator=`
- `~CharacterAction` `V`

### `CharacterActionBase` (Game.dll, 20)

- `ActionAborted` `V`
- `AnimationCallback` `V`
- `CharacterActionBase`
- `CharacterActionBase`
- `Execute` `V`
- `Finish` `V`
- `GetBlendTime` `V`
- `GetNetPacket` `V`
- `GetParentId` `V`
- `GetPreAction` `V`
- `GetType` `V`
- `IsActive` `VC`
- `LoadPreMoveData` `V`
- `PendingActionAborted` `V`
- `SetBlendTime`
- `SetPreMoveData` `V`
- `SupportsNetwork` `V`
- `ToString` `VC`
- ``vftable'`
- `~CharacterActionBase` `V`

### `CharacterActionHandler` (Game.dll, 18)

- `AnimationCallback`
- `CalculateAllocatedMemory` `C`
- `CharacterActionHandler`
- `CharacterActionHandler`
- `Execute`
- `GetActionAsString` `C`
- `GetActionType`
- `GetBlendTime`
- `GetCurrentAction`
- `IsActive`
- `QueryActionPermission` `C`
- `RecursionLimitValue` `S`
- `SetActionFilter` `V`
- `SetCurrentAction`
- `Stop`
- ``vftable'`
- `operator=`
- `~CharacterActionHandler` `V`

### `CharacterActionPacket` (Engine.dll, 9)

- `CharacterActionPacket`
- `CharacterActionPacket`
- `CopyInbound` `V`
- `Deserialize` `V`
- `PrepareOutBuffer` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~CharacterActionPacket` `V`

### `CharacterBio` (Game.dll, 56)

- `AddConstitutionFast`
- `AddLifeFast`
- `AddLifeSlow`
- `AddManaFast`
- `AddManaSlow`
- `BalanceHasChanged`
- `BioIsTainted`
- `CalculateAllocatedMemory` `C`
- `CharacterBio`
- `CharacterBio`
- `ClearRestorationHold`
- `EvaluateEquations`
- `GetAttribute` `C`
- `GetBaseCharAttribute` `C`
- `GetBaseCharAttributes` `C`
- `GetBonusLifeAmount` `C`
- `GetCharAttributes` `C`
- `GetConstitutionLimit` `C`
- `GetConstitutionPotential` `C`
- `GetCurrentBioStats` `C`
- `GetCurrentConstitution` `C`
- `GetCurrentLife` `C`
- `GetCurrentMana` `C`
- `GetDefenseAttributes` `C`
- `GetLifeLimit` `C`
- `GetLifePotential` `C`
- `GetLifeRegenValue` `C`
- `GetManaLimit` `C`
- `GetManaPotential` `C`
- `GetManaRegenValue` `C`
- `GetManaReserve` `C`
- `GetModifier` `C`
- `GetOriginalCharAttribute` `C`
- `GetTaintedBio` `C`
- `HasModifierPointsInUse` `C`
- `IncrementBaseAttribute`
- `InitialUpdate`
- `Load`
- `ReadProperties`
- `ReloadBaseAttrAcc`
- `ReloadValues`
- `ResetAttributePoints`
- `ResolveEquationVariable` `VC`
- `RestoreCurrentBioStats`
- `SetAcceleratedLifeRegen`
- `SetAttribute`
- `StreamProperties`
- `SubtractLife`
- `SubtractMana`
- `TakeBonus`
- `Update`
- `UpdateHealthAndMana`
- `UpdateResources`
- `WriteProperties` `C`
- ``vftable'`
- `~CharacterBio`

### `CharacterHookPack` (Game.dll, 6)

- `CharacterHookPack`
- `CharacterHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~CharacterHookPack` `V`

### `CharacterMovementManager` (Game.dll, 28)

- `Activate`
- `AlreadyThere` `C`
- `CharacterHasDied`
- `CharacterMovementManager`
- `CharacterMovementManager`
- `CheckForPortal` `C`
- `Deactivate`
- `DebugRender`
- `DisableMovement`
- `FindPath`
- `GetFurthestMoveToPoint`
- `GetMoveToDistance` `C`
- `GetMovementTarget` `C`
- `GetPointAwayFromGoal`
- `GetStraightMoveToPoint`
- `IsActivated` `C`
- `IsMovementDisabled` `C`
- `IsMoving` `C`
- `MoveTo`
- `MoveToNextWaypoint`
- `OnPathFailed`
- `OnReachedMovementGoal`
- `Stop`
- `Teleported`
- `Update`
- `kAlreadyThereThreshold` `S`
- `operator=`
- `~CharacterMovementManager`

### `CharonGeyserMarker` (Game.dll, 10)

- `CharonGeyserMarker`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~CharonGeyserMarker` `V`

### `ChatAction` (Game.dll, 11)

- `AnimationCallback` `V`
- `ChatAction`
- `ChatAction`
- `Execute` `V`
- `FaceTarget` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `ResetTransitionTimer`
- `ToString` `VC`
- ``vftable'`
- `~ChatAction` `V`

### `ChatActionPacket` (Game.dll, 8)

- `ChatActionPacket`
- `ChatActionPacket`
- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~ChatActionPacket` `V`

### `ChatCommandUseSkillPacket` (Game.dll, 8)

- `ChatCommandUseSkillPacket`
- `ChatCommandUseSkillPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ChatCommandUseSkillPacket` `V`

### `ChatPacket` (Engine.dll, 8)

- `ChatPacket`
- `ChatPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ChatPacket` `V`

### `ChestHookPack` (Game.dll, 6)

- `ChestHookPack`
- `ChestHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~ChestHookPack` `V`

### `CleanseShrineConfigCmd` (Game.dll, 8)

- `CleanseShrineConfigCmd`
- `CleanseShrineConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeOfferings`
- ``vftable'`
- `operator=`
- `~CleanseShrineConfigCmd` `V`

### `CleanseShrineConfigCmdPacket` (Game.dll, 8)

- `CleanseShrineConfigCmdPacket`
- `CleanseShrineConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CleanseShrineConfigCmdPacket` `V`

### `CleanseShrinePacket` (Game.dll, 8)

- `CleanseShrinePacket`
- `CleanseShrinePacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CleanseShrinePacket` `V`

### `ClearMutatorsPacket` (Game.dll, 8)

- `ClearMutatorsPacket`
- `ClearMutatorsPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ClearMutatorsPacket` `V`

### `ClientConnectionManager` (Engine.dll, 27)

- `ClientConnectionManager`
- `ClientConnectionManager`
- `ConnectToInternetHost` `V`
- `ConnectToLANServer` `V`
- `ConnectionAttemptAborted` `V`
- `CreateAddressResolver`
- `DisconnectFromServer` `V`
- `DumpHostTable` `V`
- `DumpStats` `V`
- `DumpStatsToString` `V`
- `FindLANServer` `V`
- `GetPing`
- `GetServerBrowser`
- `HandleControlSocketPacket`
- `HandlePacket` `V`
- `Initialize` `V`
- `InitializeControlSocket` `V`
- `PingServer` `V`
- `SendPacket` `V`
- `SendPacketToGroup` `V`
- `SendPacketToServer` `V`
- `Shutdown` `V`
- `StartInternet` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~ClientConnectionManager` `V`

### `ClientDisconnectPacket` (Engine.dll, 8)

- `ClientDisconnectPacket`
- `ClientDisconnectPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ClientDisconnectPacket` `V`

### `ClientEntityList` (Engine.dll, 9)

- `AddEntity` `V`
- `ClientEntityList`
- `ClientEntityList`
- `HandleCreatedEntity` `V`
- `RemoveEntity` `V`
- `RemoveEntityFromAllClients` `V`
- ``vftable'`
- `operator=`
- `~ClientEntityList` `V`

### `ClientNetworkShim` (Engine.dll, 7)

- `ClientNetworkShim`
- `ClientNetworkShim`
- `SendCharacterAction` `V`
- `SendConfigCommand` `V`
- ``vftable'`
- `operator=`
- `~ClientNetworkShim` `V`

### `ClientReadyPacket` (Engine.dll, 8)

- `ClientReadyPacket`
- `ClientReadyPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ClientReadyPacket` `V`

### `ClientServerQueryPacket` (Engine.dll, 8)

- `ClientServerQueryPacket`
- `ClientServerQueryPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ClientServerQueryPacket` `V`

### `ClimateSectorData` (Engine.dll, 8)

- `ClimateSectorData`
- `ClimateSectorData`
- `ClimateSectorData`
- `Copy` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~ClimateSectorData` `V`

### `ColorPanel` (Widget.dll, 4)

- `ColorPanel`
- `Create`
- `OnPaint` `V`
- `SetColor`

### `ColorPulse` (Engine.dll, 18)

- `ColorPulse`
- `ColorPulse`
- `ColorPulse`
- `ColorPulse`
- `GetCurrentColor` `C`
- `IsPulsing`
- `LoadFromDatabase`
- `Reset`
- `SetColorA`
- `SetColorB`
- `SetDefaultColor`
- `SetDelayTime`
- `SetPulseTime`
- `SyncToColorPulse`
- `Update`
- ``vftable'`
- `operator=`
- `~ColorPulse` `V`

### `ColorWindow` (Widget.dll, 11)

- `ColorWindow`
- `Create`
- `GetColor` `C`
- `OnChildEvent` `V`
- `OnCreate` `V`
- `OnDrawItem` `V`
- `OnMouseButton` `V`
- `OnMouseMove` `V`
- `OnSetCursor` `V`
- `SetOriginalColor`
- `~ColorWindow` `V`

### `CombatAttribute` (Game.dll, 44)

- `CombatAttribute`
- `CombatAttribute`
- `Copy` `V`
- `DamageReductionByShield` `V`
- `Execute` `V`
- `Execute` `V`
- `FilterForSameSource` `V`
- `GetConverted` `C`
- `GetDamageSourceId` `C`
- `GetDamageType` `VC`
- `GetDefense` `VC`
- `GetDuration` `VC`
- `GetExecutingTypes` `VC`
- `GetFilterForDamageMult` `VC`
- `GetFilterForReflectDamage` `VC`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalAbsorption` `VC`
- `GetTotalDamage` `V`
- `GetTotalDefense` `VC`
- `GetTotalDurationMod` `V`
- `GetTotalReduction` `V`
- `GetType` `VC`
- `GetValues` `V`
- `MergeDefense` `V`
- `MergePhysicalDamage` `V`
- `ModifyAbsoluteDamage` `V`
- `ModifyAbsoluteDefense` `V`
- `ModifyDefenseCap` `V`
- `ModifyDurationDamage` `V`
- `ModifyDurationDefense` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceAbsoluteDamage` `V`
- `ReduceDamage` `V`
- `ReduceDurationDamage` `V`
- `ScaleDamage` `V`
- `ScaleModifiers` `V`
- `SetConverted`
- `SetProcess` `V`
- `SetSkillSource` `V`
- `UseReflectCap` `VC`
- ``vftable'`
- `~CombatAttribute` `V`

### `CombatAttributeAbsDamage` (Game.dll, 21)

- `Clone` `VC`
- `CombatAttributeAbsDamage`
- `CombatAttributeAbsDamage`
- `CombatAttributeAbsDamage`
- `Copy` `V`
- `DamageReductionByShield` `V`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `GetValues` `V`
- `ModifyAbsoluteDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceAbsoluteDamage` `V`
- `ReduceDamage` `V`
- `ScaleDamage` `V`
- `ScaleModifiers` `V`
- ``vftable'`
- `~CombatAttributeAbsDamage` `V`

### `CombatAttributeAbsDamageElemental` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeAbsDamageElemental`
- `CombatAttributeAbsDamageElemental`
- `CombatAttributeAbsDamageElemental`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeAbsDamageElemental` `V`

### `CombatAttributeAbsDamageMod` (Game.dll, 8)

- `Clone` `VC`
- `CombatAttributeAbsDamageMod`
- `CombatAttributeAbsDamageMod`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetTotalDamage` `V`
- ``vftable'`
- `~CombatAttributeAbsDamageMod` `V`

### `CombatAttributeAbsDamage_Disruption` (Game.dll, 10)

- `Clone` `VC`
- `CombatAttributeAbsDamage_Disruption`
- `CombatAttributeAbsDamage_Disruption`
- `CombatAttributeAbsDamage_Disruption`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- ``vftable'`
- `~CombatAttributeAbsDamage_Disruption` `V`

### `CombatAttributeAbsDamage_LifeLeech` (Game.dll, 11)

- `Clone` `VC`
- `CombatAttributeAbsDamage_LifeLeech`
- `CombatAttributeAbsDamage_LifeLeech`
- `CombatAttributeAbsDamage_LifeLeech`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetTotalDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- ``vftable'`
- `~CombatAttributeAbsDamage_LifeLeech` `V`

### `CombatAttributeAbsDamage_ManaBurn` (Game.dll, 11)

- `Clone` `VC`
- `CombatAttributeAbsDamage_ManaBurn`
- `CombatAttributeAbsDamage_ManaBurn`
- `CombatAttributeAbsDamage_ManaBurn`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetTotalDamage` `V`
- `ModifyAbsoluteDamage` `V`
- `ReduceDamage` `V`
- ``vftable'`
- `~CombatAttributeAbsDamage_ManaBurn` `V`

### `CombatAttributeAbsDamage_PercentLife` (Game.dll, 12)

- `Clone` `VC`
- `CombatAttributeAbsDamage_PercentLife`
- `CombatAttributeAbsDamage_PercentLife`
- `CombatAttributeAbsDamage_PercentLife`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetTotalDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceDamage` `V`
- ``vftable'`
- `~CombatAttributeAbsDamage_PercentLife` `V`

### `CombatAttributeAbsDefense` (Game.dll, 15)

- `Clone` `VC`
- `CombatAttributeAbsDefense`
- `CombatAttributeAbsDefense`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetDefense` `VC`
- `GetMinMaxValues` `V`
- `GetTotalDefense` `VC`
- `GetTotalReduction` `V`
- `MergeDefense` `V`
- `ModifyAbsoluteDefense` `V`
- `ModifyDefenseCap` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeAbsDefense` `V`

### `CombatAttributeAbsDefenseMod` (Game.dll, 11)

- `Clone` `VC`
- `CombatAttributeAbsDefenseMod`
- `CombatAttributeAbsDefenseMod`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetDefense` `VC`
- `GetMinMaxValues` `V`
- `GetTotalDefense` `VC`
- `MergeDefense` `V`
- ``vftable'`
- `~CombatAttributeAbsDefenseMod` `V`

### `CombatAttributeAbsDefenseRestricted` (Game.dll, 6)

- `Clone` `VC`
- `CombatAttributeAbsDefenseRestricted`
- `CombatAttributeAbsDefenseRestricted`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeAbsDefenseRestricted` `V`

### `CombatAttributeAbsDefenseTotalSpeed` (Game.dll, 8)

- `Clone` `VC`
- `CombatAttributeAbsDefenseTotalSpeed`
- `CombatAttributeAbsDefenseTotalSpeed`
- `Execute` `V`
- `ModifyAbsoluteDefense` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeAbsDefenseTotalSpeed` `V`

### `CombatAttributeAccumulator` (Game.dll, 57)

- `AddDamage`
- `AddDamageModifier`
- `AddDefense`
- `AddDefenseModifier`
- `Clear`
- `ClearDamage`
- `ClearDamageModifiers`
- `ClearDefense`
- `ClearDefenseModifiers`
- `CombatAttributeAccumulator`
- `CombatAttributeAccumulator`
- `ConvertCombatAttributeForDisplay` `C`
- `ConvertCombatAttributes` `C`
- `ConvertDamage`
- `DamageReductionByShield`
- `EvaluateChance` `V`
- `ExecuteDamage`
- `ExecuteDefense`
- `FilterReflectDamage`
- `FilterSameSource`
- `GetChanceMultiplier` `V`
- `GetDamage` `C`
- `GetDamageMod` `C`
- `GetDamageTypes` `C`
- `GetDefense` `C`
- `GetDefenseMod` `C`
- `GetIncludeProcs` `V`
- `GetMinMaxDamage` `C`
- `GetRandomGen` `V`
- `GetTotalDamage`
- `GetTotalDamageModifierType`
- `GetTotalDamageType`
- `GetTotalDefenseModifierType`
- `GetTotalDefenseType`
- `GetTotalDurationModifierType`
- `GetTotalReduction`
- `IgnoreDamageMultiplier` `C`
- `IsConversionCompatibleType` `C`
- `IsDurationCompatibleType` `C`
- `IsEmpty` `C`
- `IsUsedForDisplay` `V`
- `MergePhysicalDamage`
- `ModifyDamage`
- `ProcessBluntDamageModifier`
- `ProcessDamage`
- `ProcessDefense`
- `ReduceAbsoluteDamage`
- `ReplaceDamage`
- `ScaleDamage`
- `SetGlobalChance` `V`
- `SetIgnoreDamageMultiplier`
- `SetRegion`
- `SetSeed`
- `SetSkillSource`
- ``vftable'`
- `operator=`
- `~CombatAttributeAccumulator` `V`

### `CombatAttributeDamage_BasePhysical` (Game.dll, 21)

- `Clone` `VC`
- `CombatAttributeDamage_BasePhysical`
- `CombatAttributeDamage_BasePhysical`
- `DamageReductionByShield` `V`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetExecutingTypes` `VC`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `GetType` `VC`
- `GetValues` `V`
- `MergePhysicalDamage` `V`
- `ModifyAbsoluteDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceDamage` `V`
- `ScaleDamage` `V`
- `ScaleModifiers` `V`
- ``vftable'`
- `~CombatAttributeDamage_BasePhysical` `V`

### `CombatAttributeDamage_BonusPhysical` (Game.dll, 20)

- `Clone` `VC`
- `CombatAttributeDamage_BonusPhysical`
- `CombatAttributeDamage_BonusPhysical`
- `CombatAttributeDamage_BonusPhysical`
- `Copy` `V`
- `DamageReductionByShield` `V`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `GetValues` `V`
- `ModifyAbsoluteDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceDamage` `V`
- `ScaleDamage` `V`
- `ScaleModifiers` `V`
- ``vftable'`
- `~CombatAttributeDamage_BonusPhysical` `V`

### `CombatAttributeDefenseCap` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeDefenseCap`
- `CombatAttributeDefenseCap`
- `Execute` `V`
- `GetMinMaxValues` `V`
- ``vftable'`
- `~CombatAttributeDefenseCap` `V`

### `CombatAttributeDefenseMisc` (Game.dll, 10)

- `Clone` `VC`
- `CombatAttributeDefenseMisc`
- `CombatAttributeDefenseMisc`
- `GetMinMaxValues` `V`
- `GetTotalDefense` `VC`
- `GetTotalReduction` `V`
- `ModifyAbsoluteDefense` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDefenseMisc` `V`

### `CombatAttributeDefenseMiscRestricted` (Game.dll, 6)

- `Clone` `VC`
- `CombatAttributeDefenseMiscRestricted`
- `CombatAttributeDefenseMiscRestricted`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDefenseMiscRestricted` `V`

### `CombatAttributeDefenseNegative` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeDefenseNegative`
- `CombatAttributeDefenseNegative`
- `Execute` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDefenseNegative` `V`

### `CombatAttributeDefense_AbsorptionProtection` (Game.dll, 12)

- `Clone` `VC`
- `CombatAttributeDefense_AbsorptionProtection`
- `CombatAttributeDefense_AbsorptionProtection`
- `Execute` `V`
- `GetTotalAbsorption` `VC`
- `GetTotalDefense` `VC`
- `GetTotalReduction` `V`
- `GetType` `VC`
- `ModifyAbsoluteDefense` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDefense_AbsorptionProtection` `V`

### `CombatAttributeDurDamage` (Game.dll, 22)

- `Clone` `VC`
- `CombatAttributeDurDamage`
- `CombatAttributeDurDamage`
- `CombatAttributeDurDamage`
- `Copy` `V`
- `DamageReductionByShield` `V`
- `Execute` `V`
- `FilterForSameSource` `V`
- `GetDamageType` `VC`
- `GetDuration` `VC`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `ModifyDurationDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceDamage` `V`
- `ReduceDurationDamage` `V`
- `ScaleDamage` `V`
- `ScaleModifiers` `V`
- ``vftable'`
- `~CombatAttributeDurDamage` `V`

### `CombatAttributeDurDamageElemental` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeDurDamageElemental`
- `CombatAttributeDurDamageElemental`
- `CombatAttributeDurDamageElemental`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDurDamageElemental` `V`

### `CombatAttributeDurDamageMod` (Game.dll, 9)

- `Clone` `VC`
- `CombatAttributeDurDamageMod`
- `CombatAttributeDurDamageMod`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetTotalDamage` `V`
- `GetTotalDurationMod` `V`
- ``vftable'`
- `~CombatAttributeDurDamageMod` `V`

### `CombatAttributeDurDefense` (Game.dll, 14)

- `Clone` `VC`
- `CombatAttributeDurDefense`
- `CombatAttributeDurDefense`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetDefense` `VC`
- `GetMinMaxValues` `V`
- `GetTotalDefense` `VC`
- `MergeDefense` `V`
- `ModifyDefenseCap` `V`
- `ModifyDurationDefense` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeDurDefense` `V`

### `CombatAttributeDurDefenseMod` (Game.dll, 6)

- `Clone` `VC`
- `CombatAttributeDurDefenseMod`
- `CombatAttributeDurDefenseMod`
- `Execute` `V`
- ``vftable'`
- `~CombatAttributeDurDefenseMod` `V`

### `CombatAttributeDurFixedDamage` (Game.dll, 11)

- `Clone` `VC`
- `CombatAttributeDurFixedDamage`
- `CombatAttributeDurFixedDamage`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetTotalDamage` `V`
- `Process` `V`
- `ProcessBluntDamageModifier` `V`
- `ReduceAbsoluteDamage` `V`
- ``vftable'`
- `~CombatAttributeDurFixedDamage` `V`

### `CombatAttributeDur_DamageMultiplier` (Game.dll, 10)

- `Clone` `VC`
- `CombatAttributeDur_DamageMultiplier`
- `CombatAttributeDur_DamageMultiplier`
- `Execute` `V`
- `GetDamageType` `VC`
- `ModifyDurationDamage` `V`
- `Process` `V`
- `ScaleDamage` `V`
- ``vftable'`
- `~CombatAttributeDur_DamageMultiplier` `V`

### `CombatAttributeInfluenceDamage` (Game.dll, 10)

- `Clone` `VC`
- `CombatAttributeInfluenceDamage`
- `CombatAttributeInfluenceDamage`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `ReduceAbsoluteDamage` `V`
- ``vftable'`
- `~CombatAttributeInfluenceDamage` `V`

### `CombatAttributeInfluenceDamage_Confusion` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeInfluenceDamage_Confusion`
- `CombatAttributeInfluenceDamage_Confusion`
- `CombatAttributeInfluenceDamage_Confusion`
- `Execute` `V`
- ``vftable'`
- `~CombatAttributeInfluenceDamage_Confusion` `V`

### `CombatAttributeInfluenceDamage_Convert` (Game.dll, 8)

- `Clone` `VC`
- `CombatAttributeInfluenceDamage_Convert`
- `CombatAttributeInfluenceDamage_Convert`
- `CombatAttributeInfluenceDamage_Convert`
- `Execute` `V`
- `Process` `V`
- ``vftable'`
- `~CombatAttributeInfluenceDamage_Convert` `V`

### `CombatAttributeInfluenceDamage_Fear` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeInfluenceDamage_Fear`
- `CombatAttributeInfluenceDamage_Fear`
- `CombatAttributeInfluenceDamage_Fear`
- `Execute` `V`
- ``vftable'`
- `~CombatAttributeInfluenceDamage_Fear` `V`

### `CombatAttributeInfluenceDamage_Taunt` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttributeInfluenceDamage_Taunt`
- `CombatAttributeInfluenceDamage_Taunt`
- `CombatAttributeInfluenceDamage_Taunt`
- `Execute` `V`
- ``vftable'`
- `~CombatAttributeInfluenceDamage_Taunt` `V`

### `CombatAttributeReflexDamage` (Game.dll, 10)

- `Clone` `VC`
- `CombatAttributeReflexDamage`
- `CombatAttributeReflexDamage`
- `Execute` `V`
- `GetMinMaxDamage` `V`
- `GetMinMaxValues` `V`
- `GetTotalDamage` `V`
- `ReduceAbsoluteDamage` `V`
- ``vftable'`
- `~CombatAttributeReflexDamage` `V`

### `CombatAttributeTotalDamageMod` (Game.dll, 6)

- `Clone` `VC`
- `CombatAttributeTotalDamageMod`
- `CombatAttributeTotalDamageMod`
- `Execute` `V`
- ``vftable'`
- `~CombatAttributeTotalDamageMod` `V`

### `CombatAttribute_DamageCritBonus` (Game.dll, 7)

- `Clone` `VC`
- `CombatAttribute_DamageCritBonus`
- `CombatAttribute_DamageCritBonus`
- `GetDamageType` `VC`
- `GetTotalDamage` `V`
- ``vftable'`
- `~CombatAttribute_DamageCritBonus` `V`

### `CombatAttribute_DamageMultiplier` (Game.dll, 8)

- `Clone` `VC`
- `CombatAttribute_DamageMultiplier`
- `CombatAttribute_DamageMultiplier`
- `Execute` `V`
- `GetDamageType` `VC`
- `GetMinMaxValues` `V`
- ``vftable'`
- `~CombatAttribute_DamageMultiplier` `V`

### `CombatDisplayAccumulator` (Game.dll, 14)

- `CombatDisplayAccumulator`
- `CombatDisplayAccumulator`
- `EvaluateChance` `V`
- `GetChanceMultiplier` `V`
- `GetModifyProcs` `V`
- `GetRandomGen` `V`
- `SetGlobalChance` `V`
- `SetIncludeProcs`
- `SetModifyProcs`
- `SetProcess`
- ``default constructor closure'`
- ``vftable'`
- `operator=`
- `~CombatDisplayAccumulator` `V`

### `CombatManager` (Game.dll, 82)

- `AddCombatFilter`
- `ApplyDamage`
- `ApplyLifeLeech`
- `AssignParent`
- `AttachItemAction`
- `CalculateAllocatedMemory` `C`
- `CalculateDamageModifier` `C`
- `CanIDeflectProjectile` `C`
- `CanIDodgeAttack` `C`
- `CombatManager`
- `ContributeDamageMultiplier` `C`
- `ContributeDefensiveAbility` `C`
- `ContributeDefensiveReduction` `C`
- `ContributeElementalDamageReductionPercent` `C`
- `ContributeElementalResistanceReductionAbsolute` `C`
- `ContributeElementalResistanceReductionPercent` `C`
- `ContributeFumbleDamage` `C`
- `ContributeOffensiveAbility` `C`
- `ContributeOffensiveReduction` `C`
- `ContributePhysicalDamageReductionPercent` `C`
- `ContributePhysicalResistanceReductionAbsolute` `C`
- `ContributePhysicalResistanceReductionPercent` `C`
- `ContributeProjectileFumbleDamage` `C`
- `ContributeSpellCastSpeed` `C`
- `ContributeTotalDamageReductionAbsolute` `C`
- `ContributeTotalDamageReductionPercent` `C`
- `ContributeTotalResistanceReductionAbsolute` `C`
- `ContributeTotalResistanceReductionPercent` `C`
- `DesignerCalculateCriticalChance` `C`
- `DesignerCalculateDefensiveAbility` `C`
- `DesignerCalculateMagicalDamage` `C`
- `DesignerCalculateMagicalDurationDamage` `C`
- `DesignerCalculateMeleeBlockChance` `C`
- `DesignerCalculateOffensiveAbility` `C`
- `DesignerCalculatePhysicalDamage` `C`
- `DesignerCalculatePhysicalDamageBonus` `C`
- `DesignerCalculatePhysicalDamageDefense` `C`
- `DesignerCalculatePhysicalDamagePercentage` `C`
- `DesignerCalculatePhysicalDurationDamage` `C`
- `DesignerCalculatePierceDamage` `C`
- `DesignerCalculateProbabilityToHit` `C`
- `DesignerCalculateProjectileBlockChance` `C`
- `DesignerCalculateShieldBlockDamageReduction` `C`
- `DetachItemAction`
- `ForceSpeedUpdate`
- `GetAttachCoords` `C`
- `GetAttachedItems` `C`
- `GetAttackSpeed` `C`
- `GetAttackerId` `C`
- `GetCharacter`
- `GetCombatState` `C`
- `GetDurationDamageMgr`
- `GetEquipManager`
- `GetEquipManager` `C`
- `GetEquipmentArmorId` `C`
- `GetHandHitDamage`
- `GetHandHitDamage` `C`
- `GetHandState` `C`
- `GetLastAttackTime` `C`
- `GetLeftHandWeapon` `C`
- `GetRegionChance` `C`
- `GetRightHandWeapon` `C`
- `GetRunSpeed` `C`
- `GiveBonus`
- `ImDead`
- `ImDying`
- `IsPoisoned` `C`
- `Load`
- `LoadRecord`
- `PickRegion` `C`
- `PreLoad`
- `RecordHit`
- `ReflectCombatDamage`
- `RemoveCombatFilter`
- `ResolveEquationVariable` `VC`
- `Retaliation`
- `SetAttacker`
- `SetCombatState`
- `TakeAttack`
- `Update`
- ``vftable'`
- `~CombatManager` `V`

### `CompleteRelicsConfigCmd` (Game.dll, 6)

- `CompleteRelicsConfigCmd`
- `CompleteRelicsConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `~CompleteRelicsConfigCmd` `V`

### `CompleteRelicsConfigCmdPacket` (Game.dll, 8)

- `CompleteRelicsConfigCmdPacket`
- `CompleteRelicsConfigCmdPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CompleteRelicsConfigCmdPacket` `V`

### `ConfigFile` (Engine.dll, 10)

- `Begin` `C`
- `End` `C`
- `GetValue` `C`
- `GetValueAsBool` `C`
- `GetValueAsInt` `C`
- `GetValueAsInt2` `C`
- `GetValueAsStr` `C`
- `GetValueAsfloat` `C`
- `Initialize`
- `Initialize`

### `ConnectionManager` (Engine.dll, 67)

- `AuthorizeMe`
- `CheckPublicIP`
- `CompleteSteamAuth` `V`
- `ConnectionManager`
- `ConnectionManager`
- `DisconnectFromServer` `V`
- `DisconnectHost` `V`
- `DumpHostTable` `V`
- `DumpStats` `V`
- `DumpStatsToString` `V`
- `EnableHeartbeat`
- `FindLocalHostName`
- `FindLocalIP`
- `FlushConnection` `V`
- `GetGlobalMaxPlayers`
- `GetKeyHash`
- `GetListeningPort`
- `GetLocalHostID`
- `GetLocalIP` `C`
- `GetNetworkAdapters` `S`
- `GetNetworkEntityList` `V`
- `GetNetworkFrameStats` `C`
- `GetNetworkMode`
- `GetNetworkTotalFrameStats` `C`
- `GetNextEphemeralPort` `C`
- `GetOfflineHostID`
- `GetPing`
- `GetPingForHost`
- `GetPublicIP` `C`
- `GetRecommendedNetworkAdapterAddress` `S`
- `GetSteamAuthProvider` `C`
- `GetSteamAuthRequester` `C`
- `GlobalEnableNetwork` `V`
- `HandlePacket` `V`
- `Initialize` `V`
- `InitializeControlSocket` `V`
- `IsBanned`
- `IsConnectedToServer`
- `IsHostConnected`
- `IsInitialized`
- `IsReadyForGameLoad` `V`
- `IsReceiveOk`
- `IsSendOk`
- `IsSocketErrorFatal`
- `ReceiveControlPacket`
- `RemoveEntity` `V`
- `ResetServerConnection`
- `SendControlPacket`
- `SendControlPackets`
- `SendPacket` `V`
- `SendPacketExcluding` `V`
- `SendPacketToGroup` `V`
- `SendPacketToHost` `V`
- `SendPacketToServer` `V`
- `SetEntityList`
- `SetMaxHosts`
- `SetNetworkLogging`
- `SetNetworkStatisticsGathering`
- `Shutdown` `V`
- `StartInternet` `V`
- `StartLAN` `V`
- `SubtractPacketCounts`
- `Update` `V`
- `UpdateFrameStats`
- ``vftable'`
- `operator=`
- `~ConnectionManager` `V`

### `ConnectionVerifyPacket` (Engine.dll, 8)

- `ConnectionVerifyPacket`
- `ConnectionVerifyPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ConnectionVerifyPacket` `V`

### `ControlPlayerRespawnPacket` (Game.dll, 8)

- `ControlPlayerRespawnPacket`
- `ControlPlayerRespawnPacket`
- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ControlPlayerRespawnPacket` `V`

### `Controller` (Game.dll, 12)

- `AttachParent` `V`
- `Controller`
- `Controller`
- `GetParentId` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PostParentAttached` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Controller` `V`

### `ControllerAI` (Game.dll, 114)

- `AddPreloadQuestAction`
- `AddState`
- `AddTemporaryState`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `BeginDeath` `V`
- `CanMove` `VC`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `ControllerAI`
- `ControllerAI`
- `DebugRender` `V`
- `Die` `V`
- `EndOfPathReached` `V`
- `Evade`
- `FactionChanged` `V`
- `FindClosest` `C`
- `FindClosest` `C`
- `FindStrongest` `C`
- `FindWeakest` `C`
- `GetAI` `C`
- `GetAllowQuestInterruptions` `C`
- `GetCurrentAlly` `C`
- `GetCurrentEnemy` `C`
- `GetCurrentPointOfInterest` `C`
- `GetCurrentSkillID` `C`
- `GetCurrentStateData` `C`
- `GetExecutingState` `C`
- `GetExecutingStateName` `C`
- `GetMoveTerrainPoint` `C`
- `GetPathingViewDistance` `C`
- `GetQuestAnimation` `C`
- `GetQuestAnimationIdTag` `C`
- `GetQuestAnimationLooping` `C`
- `GetQuestMoveIdTag` `C`
- `GetQuestMoveIdTagNext` `C`
- `GetRTTIClassInfo` `VC`
- `GetState`
- `GetStaticClassInfo` `S`
- `GetViewDistance` `C`
- `GoToIdleState`
- `HandleEvent` `V`
- `Idle`
- `Immobilize`
- `InitialUpdate` `V`
- `InitializeStates` `V`
- `IsAvailableForSidelineConversations`
- `IsInState`
- `IsMoving` `C`
- `JumpToUseSkill`
- `KnockMeDown` `V`
- `Load` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd`
- `NotifySidelineDialogThread`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RegisterTemporaryStates` `V`
- `RemoveImmobilize`
- `RemoveKnockdown`
- `RemoveTrap`
- `RequestAttack`
- `RequestConversation`
- `RequestMove`
- `RequestSidelineConversation`
- `RotateTowards`
- `SetCausesAnger`
- `SetConversationDisinterest`
- `SetFaction`
- `SetInvincible`
- `SetMoveTerrainPoint`
- `SetQuestAnimation`
- `SetQuestAnimationIdTag`
- `SetQuestMoveIdTag`
- `SetQuestMoveIdTagNext`
- `SetState`
- `SetViewDistance`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `TakeTrap`
- `TauntMe` `V`
- `UnderAttack` `V`
- `Update` `V`
- `Use`
- `UseSkill`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `classInfo` `S`
- `~ControllerAI` `V`

### `ControllerAIState` (Game.dll, 5)

- `ControllerAIState`
- `ControllerAIState`
- ``vftable'`
- `operator=`
- `~ControllerAIState` `V`

### `ControllerAIStateData` (Game.dll, 8)

- `ControllerAIStateData`
- `ControllerAIStateData`
- `GetAllyID` `C`
- `GetEnemyID` `C`
- `GetPointOfInterest` `C`
- `GetSkillID` `C`
- `operator=`
- `~ControllerAIStateData`

### `ControllerAIStatePreStart` (Game.dll, 13)

- `ControllerAIStatePreStart`
- `ControllerAIStatePreStart`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerAIStatePreStart` `V`

### `ControllerAIStateT<GAME::ControllerAI,GAME::Character>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerAI,GAME::Character>`
- `ControllerAIStateT<GAME::ControllerAI,GAME::Character>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerAI,GAME::Character>` `V`

### `ControllerAIStateT<GAME::ControllerCerberus,GAME::Cerberus>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerCerberus,GAME::Cerberus>`
- `ControllerAIStateT<GAME::ControllerCerberus,GAME::Cerberus>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerCerberus,GAME::Cerberus>` `V`

### `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Megalesios>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Megalesios>`
- `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Megalesios>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerMegalesios,GAME::Megalesios>` `V`

### `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerMegalesios,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerMegalesios,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerMonster,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerMonster,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerMonster,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerMonster,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerMonsterHidden,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerMonsterHidden,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerMonsterHidden,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerMonsterHidden,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerMonsterSynergy,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerMonsterSynergy,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerMonsterSynergy,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerMonsterSynergy,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerNpc2,GAME::Npc>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerNpc2,GAME::Npc>`
- `ControllerAIStateT<GAME::ControllerNpc2,GAME::Npc>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerNpc2,GAME::Npc>` `V`

### `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Character>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Character>`
- `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Character>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Character>` `V`

### `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Npc>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Npc>`
- `ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Npc>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerNpcHerdAnimal,GAME::Npc>` `V`

### `ControllerAIStateT<GAME::ControllerNpcHerder,GAME::Character>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerNpcHerder,GAME::Character>`
- `ControllerAIStateT<GAME::ControllerNpcHerder,GAME::Character>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerNpcHerder,GAME::Character>` `V`

### `ControllerAIStateT<GAME::ControllerPlayer,GAME::Player>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerPlayer,GAME::Player>`
- `ControllerAIStateT<GAME::ControllerPlayer,GAME::Player>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerPlayer,GAME::Player>` `V`

### `ControllerAIStateT<GAME::ControllerSpirit,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerSpirit,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerSpirit,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerSpirit,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerSpiritHost,GAME::SpiritHost>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerSpiritHost,GAME::SpiritHost>`
- `ControllerAIStateT<GAME::ControllerSpiritHost,GAME::SpiritHost>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerSpiritHost,GAME::SpiritHost>` `V`

### `ControllerAIStateT<GAME::ControllerStationaryMonster,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerStationaryMonster,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerStationaryMonster,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerStationaryMonster,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerTerracotta,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerTerracotta,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerTerracotta,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerTerracotta,GAME::Monster>` `V`

### `ControllerAIStateT<GAME::ControllerTyphonChained,GAME::Monster>` (Game.dll, 118)

- `ActivateSuperSkill` `V`
- `AddTemporaryState`
- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BlocksPath`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `Chat`
- `ChatCommandUseSkill` `V`
- `ClearTemporaryStates`
- `CloseEnoughToUseSkill` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerAIStateT<GAME::ControllerTyphonChained,GAME::Monster>`
- `ControllerAIStateT<GAME::ControllerTyphonChained,GAME::Monster>`
- `DebugRender` `V`
- `Die`
- `Done` `V`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndOfPathReached` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EnemyFound` `V`
- `Evade`
- `FindAlliesInSight`
- `FindAlliesOfType`
- `FindEnemiesInSight`
- `GetCharacter`
- `GetCharacter`
- `GetClosest`
- `GetController`
- `GetController`
- `GetCurrentAlly`
- `GetCurrentEnemy`
- `GetCurrentSkillID`
- `GetPointOfInterest`
- `GetSkillUseTolerance` `V`
- `GetViewDistance`
- `HandleEvent` `V`
- `Idle`
- `IsAlive`
- `IsAvailableForSidelineConversations` `V`
- `IsPathClear`
- `JumpToUseSkill`
- `KillMe`
- `KnockMeDown` `V`
- `LongIdle`
- `LookAt`
- `LostSlot` `V`
- `LowHealth` `V`
- `MaxSkillDistance` `V`
- `MenuIdle`
- `MoveAndUseSkill`
- `MoveTo`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `Panic` `V`
- `PathFailed` `V`
- `PickUp`
- `PlayAnimation`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RequestAttack` `V`
- `RequestCompleteRelics` `V`
- `RequestConversation` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMove` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestSidelineConversation` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- `RespondsToFear` `V`
- `RotateTowards`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `SetDone` `V`
- `SetState`
- `ShouldFindClosestEnemy` `V`
- `ShouldFindEnemy` `V`
- `SkillFailed` `V`
- `SkillUsed` `V`
- `StunMe` `V`
- `TakeHit`
- `TakeKnockdown`
- `TakeSleep`
- `TakeStun`
- `Use`
- `UsePathPositionAsHome` `V`
- `UseSkill`
- `ValidateObjectId`
- `WakeUp` `V`
- `WalkTo`
- ``vftable'`
- `operator=`
- `~ControllerAIStateT<GAME::ControllerTyphonChained,GAME::Monster>` `V`

### `ControllerAlly` (Game.dll, 11)

- `ControllerAlly`
- `ControllerAlly`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RegisterTemporaryStates` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ControllerAlly` `V`

### `ControllerBaseCharacter` (Game.dll, 41)

- `CharacterHandlerUpdate` `V`
- `CheckAction` `VC`
- `ControllerBaseCharacter`
- `ControllerBaseCharacter`
- `DebugRender` `V`
- `EndOfPathReached` `V`
- `FindClosestActor` `C`
- `FlushDebugRender`
- `GetActionsAsStrings` `C`
- `GetClosestMovePoint` `V`
- `GetPendingAction`
- `GetQueuedAction`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTarget` `VC`
- `HandleAction`
- `HandleEvent` `V`
- `InitialUpdate` `V`
- `Load` `V`
- `LocalHandleAction`
- `LostSlot` `V`
- `LowHealth` `V`
- `PathFailed` `V`
- `PeekQueuedAction` `C`
- `PickupItem` `V`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `PushDebugRenderText`
- `RTTI_new` `S`
- `RespawnMe` `V`
- `SetPendingAction`
- `SetQueuedAction`
- `SetTarget` `V`
- `Teleport` `V`
- `TriggerFired` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~ControllerBaseCharacter` `V`

### `ControllerCerberus` (Game.dll, 16)

- `ControllerCerberus`
- `ControllerCerberus`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsRoaring` `C`
- `Load` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RoarDone`
- `StartRoar`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ControllerCerberus` `V`

### `ControllerCerberusStateAttack` (Game.dll, 6)

- `ControllerCerberusStateAttack`
- `ControllerCerberusStateAttack`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerCerberusStateAttack` `V`

### `ControllerCerberusStateRoar` (Game.dll, 9)

- `ControllerCerberusStateRoar`
- `ControllerCerberusStateRoar`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerCerberusStateRoar` `V`

### `ControllerCharacter` (Game.dll, 47)

- `AllowDualWieldWeapons`
- `ControllerCharacter`
- `ControllerCharacter`
- `CreateItemInInventory` `V`
- `DecrementCharacterDexterity`
- `DecrementCharacterIntelligence`
- `DecrementCharacterLife`
- `DecrementCharacterMana`
- `DecrementCharacterStrength`
- `GetAlternateEquipment` `C`
- `GetEquipmentCtrl`
- `GetEquipmentCtrl` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementCharacterDexterity`
- `IncrementCharacterIntelligence`
- `IncrementCharacterLife`
- `IncrementCharacterMana`
- `IncrementCharacterMana`
- `IncrementCharacterStrength`
- `InitialUpdate` `V`
- `LoadLootFromActor` `V`
- `PickupItem` `V`
- `RTTI_new` `S`
- `ResetAttributePoints`
- `SendAddItemToInventory`
- `SendAltarOfferCmd`
- `SendCleanseShrineCmd`
- `SendCreateArtifactCmd`
- `SendDropItemRandom`
- `SendEnchanterDismantleCmd`
- `SendEnchanterRecoveryCmd`
- `SendEnchanterTinkerCmd`
- `SendEquipAttachAction`
- `SendEquipDetachAction`
- `SendPositionUpdate`
- `SendReclaimDevotionPointCmd`
- `SendRemoveItemFromInventory`
- `SendTransmuteItemsCmd`
- `SendUpdateItemStack`
- `SetAlternateEquipment`
- `SetIgnoreEquipmentRequirements`
- `Teleport` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~ControllerCharacter` `V`

### `ControllerCombat` (Game.dll, 37)

- `AllyAttacked` `V`
- `BeginDeath` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `ControllerCombat`
- `ControllerCombat`
- `DeathUpdate` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HandleEvent` `V`
- `InitialUpdate` `V`
- `KillMe` `V`
- `KnockMeDown` `V`
- `NeedsDeathUpdate` `VC`
- `RTTI_new` `S`
- `ScareMe` `V`
- `SetPrimarySkillId`
- `SetSecondarySkillId`
- `StunMe` `V`
- `TauntMe` `V`
- `UnderAttack` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~ControllerCombat` `V`

### `ControllerEvent` (Engine.dll, 3)

- `ControllerEvent`
- `operator=`
- `operator=`

### `ControllerGraeae` (Game.dll, 24)

- `AddToList`
- `ControllerGraeae`
- `ControllerGraeae`
- `Dead`
- `GetEye`
- `GetNextGraeae`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GiveEye`
- `HasEye` `C`
- `InitialUpdate` `V`
- `Load` `V`
- `LoseEye`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RemoveFromList`
- `ToggleEyeSkill`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `s_bFirstFire` `S`
- `s_vGraeaes` `S`
- `~ControllerGraeae` `V`

### `ControllerGraeaeStateAttack` (Game.dll, 8)

- `ControllerGraeaeStateAttack`
- `ControllerGraeaeStateAttack`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerGraeaeStateAttack` `V`

### `ControllerGraeaeStateDead` (Game.dll, 6)

- `ControllerGraeaeStateDead`
- `ControllerGraeaeStateDead`
- `OnBegin` `V`
- ``vftable'`
- `operator=`
- `~ControllerGraeaeStateDead` `V`

### `ControllerGraeaeStateDying` (Game.dll, 6)

- `ControllerGraeaeStateDying`
- `ControllerGraeaeStateDying`
- `OnBegin` `V`
- ``vftable'`
- `operator=`
- `~ControllerGraeaeStateDying` `V`

### `ControllerGraeaeStateIdle` (Game.dll, 8)

- `ControllerGraeaeStateIdle`
- `ControllerGraeaeStateIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerGraeaeStateIdle` `V`

### `ControllerGraeaeStateToggleEye` (Game.dll, 9)

- `ControllerGraeaeStateToggleEye`
- `ControllerGraeaeStateToggleEye`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerGraeaeStateToggleEye` `V`

### `ControllerHades` (Game.dll, 14)

- `ControllerHades`
- `ControllerHades`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `m_nPrevHealth` `S`
- `~ControllerHades` `V`

### `ControllerMegalesiosStateLaunchBursts` (Game.dll, 10)

- `ControllerMegalesiosStateLaunchBursts`
- `ControllerMegalesiosStateLaunchBursts`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandUseSkill` `V`
- ``vftable'`
- `operator=`
- `~ControllerMegalesiosStateLaunchBursts` `V`

### `ControllerMegalesiosStateStartup` (Game.dll, 9)

- `ControllerMegalesiosStateStartup`
- `ControllerMegalesiosStateStartup`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandUseSkill` `V`
- ``vftable'`
- `operator=`
- `~ControllerMegalesiosStateStartup` `V`

### `ControllerMonster` (Game.dll, 196)

- `AllyAttacked` `V`
- `AngerUpdate` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CanFlee` `C`
- `CanMove` `VC`
- `CanPlayStartupAnim` `C`
- `ChooseBestAllyToBuff` `C`
- `ChooseBestBuffSelfSkill` `C`
- `ChooseBestSkill`
- `ChooseBestSkillOverride` `VC`
- `ClearAnger`
- `ClosestEnemyFoundOverride` `V`
- `Confuse`
- `ControllerMonster`
- `ControllerMonster`
- `DebugRender` `V`
- `DieIfEnemyInRange`
- `DropLoot`
- `EnableSkills`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `FactionChanged` `V`
- `FindClosestEnemy`
- `FindEnemy` `V`
- `FindEnemyUpdate`
- `GetAngerManager`
- `GetAngerTolerance` `C`
- `GetBerserkSkillId` `C`
- `GetBuffAllyBehavior` `C`
- `GetBuffOther2SkillId` `C`
- `GetBuffOther3SkillId` `C`
- `GetBuffOtherSkillId` `C`
- `GetBuffSelf2SkillId` `C`
- `GetBuffSelf3SkillId` `C`
- `GetBuffSelfBehavior` `C`
- `GetBuffSelfSkillId` `C`
- `GetChainBehavior` `C`
- `GetChainInitialSkill` `C`
- `GetChainNextSkill` `C`
- `GetChanceToIdleOnPatrol` `C`
- `GetChanceToRespondToDistressCall` `C`
- `GetCharactersInSphere` `C`
- `GetClearAngerWhenFleeing` `C`
- `GetConfusedTime` `C`
- `GetCurrentPatrolPoint` `C`
- `GetDebuffEnemyBehavior` `C`
- `GetDefaultSkillId` `C`
- `GetDistressResponseBehavior` `C`
- `GetDistressResponseGroup` `C`
- `GetDodgeChance` `C`
- `GetDodgeDistance` `C`
- `GetDropMiscItems`
- `GetDyingSkillCallback` `C`
- `GetDyingSkillId` `C`
- `GetEnemyIgnoringMostHated` `C`
- `GetEnemyTooCloseDistance` `C`
- `GetFleeBehavior` `C`
- `GetFleeChance` `C`
- `GetFleeDistance` `C`
- `GetFleeTarget` `C`
- `GetFleeTime` `C`
- `GetFoesInSphere` `C`
- `GetFollowersJoined` `C`
- `GetFriendsInSphere` `C`
- `GetHealAllyHealthPercentage` `C`
- `GetHealLeaderHealthPercentage` `C`
- `GetHealSkillId` `C`
- `GetHomePosition` `C`
- `GetIgnorePetChance` `C`
- `GetIgnorePetInterval` `C`
- `GetInitial2SkillId` `C`
- `GetInitialSkillId` `C`
- `GetInnerViewDistance` `C`
- `GetLeaderBehavior` `C`
- `GetLeaderDistance` `C`
- `GetLifeTime` `C`
- `GetLongRangeMax` `C`
- `GetLongRangeMin` `C`
- `GetLootDropCallback` `C`
- `GetLootDropCoords` `VC`
- `GetLootDropGroup` `VC`
- `GetLootDropRadius` `VC`
- `GetMaxFollowers` `C`
- `GetMaxPatrolIdleTime` `C`
- `GetMaxPursuitDistance` `C`
- `GetMaxTimeBeforeRoam` `C`
- `GetMaxYViewDistance` `C`
- `GetMediumRangeMax` `C`
- `GetMediumRangeMin` `C`
- `GetMinDodgeDistance` `C`
- `GetMinPatrolIdleTime` `C`
- `GetMinRoamDistance` `C`
- `GetMinTimeBeforeRoam` `C`
- `GetMinWanderDistance` `C`
- `GetMonster` `C`
- `GetMostHatedEnemy` `V`
- `GetNormalSkillId` `C`
- `GetPatrolPoints` `C`
- `GetProxy` `C`
- `GetPursuitTime` `C`
- `GetRTTIClassInfo` `VC`
- `GetRandomRepositionChance` `C`
- `GetRepositionChance` `C`
- `GetResetOriginAfterFleeing` `C`
- `GetRoamBehavior` `C`
- `GetRoamDistance` `C`
- `GetShortRangeMax` `C`
- `GetShortRangeMin` `C`
- `GetSpecialAttackSkillId` `C`
- `GetSpecialAttackSkillInfo`
- `GetStaticClassInfo` `S`
- `GetSwingTimer`
- `GetTeleportToLeaderDistance` `C`
- `GetTreasureProxy` `C`
- `GetWanderDistance` `C`
- `HasLootDropped` `C`
- `IgnoreSleepingEnemies` `C`
- `InPursuitRange` `C`
- `IncrementFleeCount`
- `InitializeStates` `V`
- `IsEnemyValid` `C`
- `IsSkillInProperRange` `C`
- `LeadTarget`
- `Load` `V`
- `NeedsDeathUpdate` `VC`
- `NewLeader`
- `PickRandomEnemyInView` `C`
- `PostParentAttached` `V`
- `ProcessAngerTarget` `V`
- `ProjectileNotification` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RegisterTemporaryStates` `V`
- `ReplacePatrolPoints`
- `ResetFleeTimer`
- `ResetSwingTimer`
- `ScareMe` `V`
- `SetBerserkSkill`
- `SetBuffOther2Skill`
- `SetBuffOther3Skill`
- `SetBuffOtherSkill`
- `SetBuffSelf2Skill`
- `SetBuffSelf3Skill`
- `SetBuffSelfSkill`
- `SetChainBehavior`
- `SetChainInitialSkill`
- `SetChainNextSkill`
- `SetConfusedTime`
- `SetCurrentPatrolPoint`
- `SetDefaultAttackSkill`
- `SetDistressResponseGroup`
- `SetDyingSkill`
- `SetFollowersJoined`
- `SetHealSkill`
- `SetInitial2Skill`
- `SetInitialSkill`
- `SetLifetime`
- `SetMostHatedEnemy`
- `SetNeedsDeathUpdate`
- `SetNormalAttackSkill`
- `SetPlayStartupAnim`
- `SetProxyId`
- `SetRoamBehavior`
- `SetSkillRanges`
- `SetSpecialAttackSkill`
- `SetTreasureProxy`
- `ShouldAlwaysLookForClosestEnemy` `VC`
- `ShouldCareAboutDistressCall` `VC`
- `ShouldHealAlly`
- `ShouldIgnorePets` `C`
- `ShouldPlayRallyOrAlert`
- `ShowAngerLevels` `C`
- `SkillFailed`
- `StillScared` `C`
- `TauntMe` `V`
- `TeleportToLeader`
- `Teleported` `V`
- `TransferAnger` `V`
- `TryToHealAlly`
- `UnderAttack` `V`
- `Update` `V`
- `WriteSimulationInformation`
- `ZeroSwingTimer`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ControllerMonster` `V`

### `ControllerMonsterState<GAME::ControllerCerberus,GAME::Cerberus>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerCerberus,GAME::Cerberus>`
- `ControllerMonsterState<GAME::ControllerCerberus,GAME::Cerberus>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerCerberus,GAME::Cerberus>` `V`

### `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Megalesios>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Megalesios>`
- `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Megalesios>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerMegalesios,GAME::Megalesios>` `V`

### `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerMegalesios,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerMegalesios,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerMonster,GAME::Monster>` (Game.dll, 40)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CallForFollowers`
- `ControllerMonsterState<GAME::ControllerMonster,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerMonster,GAME::Monster>`
- `DefaultAllyAttackedResponse`
- `DefaultAllyDiedResponse`
- `DefaultAllyNeedsHelpResponse`
- `DefaultAttackedResponse`
- `DefaultClosestEnemyFoundResponse`
- `DefaultConfusedResponse`
- `DefaultEnemyFoundResponse`
- `DefaultLowHealthResponse`
- `DefaultPanicResponse`
- `DefaultProjectileNotificationResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `DefaultRequestAttackResponse`
- `DefaultRequestConversationResponse`
- `DefaultRequestMoveResponse`
- `HealAllyWhenAttacked`
- `JoinLeader`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `ShouldBuffSelf`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerMonster,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerMonsterHidden,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerMonsterHidden,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerMonsterHidden,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerMonsterHidden,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerMonsterSynergy,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerMonsterSynergy,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerMonsterSynergy,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerMonsterSynergy,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerSpirit,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerSpirit,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerSpirit,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerSpirit,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerSpiritHost,GAME::SpiritHost>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerSpiritHost,GAME::SpiritHost>`
- `ControllerMonsterState<GAME::ControllerSpiritHost,GAME::SpiritHost>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerSpiritHost,GAME::SpiritHost>` `V`

### `ControllerMonsterState<GAME::ControllerStationaryMonster,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerStationaryMonster,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerStationaryMonster,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerStationaryMonster,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerTerracotta,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerTerracotta,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerTerracotta,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerTerracotta,GAME::Monster>` `V`

### `ControllerMonsterState<GAME::ControllerTyphonChained,GAME::Monster>` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterState<GAME::ControllerTyphonChained,GAME::Monster>`
- `ControllerMonsterState<GAME::ControllerTyphonChained,GAME::Monster>`
- `DefaultPanicResponse`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandUseSkillResponse`
- `DefaultQuestCommandWalkResponse`
- `Panic` `V`
- `QuestCommandMove` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RespondsToFear` `V`
- `SkillUsed` `V`
- `WakeUp` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterState<GAME::ControllerTyphonChained,GAME::Monster>` `V`

### `ControllerMonsterStateAlertBeforePursue` (Game.dll, 9)

- `ControllerMonsterStateAlertBeforePursue`
- `ControllerMonsterStateAlertBeforePursue`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateAlertBeforePursue` `V`

### `ControllerMonsterStateAttack` (Game.dll, 27)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `AttackEnemyOrReturn`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateAttack`
- `ControllerMonsterStateAttack`
- `EnemyFound` `V`
- `HandleEvent` `V`
- `LostSlot` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ProjectileCollisionCallback` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `SkillFailed` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateAttack` `V`

### `ControllerMonsterStateCharge` (Game.dll, 6)

- `ControllerMonsterStateCharge`
- `ControllerMonsterStateCharge`
- `OnBegin` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateCharge` `V`

### `ControllerMonsterStateConfused` (Game.dll, 13)

- `CalculateTimeUntilRandomAttack`
- `ControllerMonsterStateConfused`
- `ControllerMonsterStateConfused`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ShouldFindEnemy` `VC`
- `WalkRandomly`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateConfused` `V`

### `ControllerMonsterStateDead` (Game.dll, 15)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterStateDead`
- `ControllerMonsterStateDead`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RespondsToFear` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateDead` `V`

### `ControllerMonsterStateDefendLeader` (Game.dll, 21)

- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateDefendLeader`
- `ControllerMonsterStateDefendLeader`
- `EnemyFound` `V`
- `GetNewTestDistanceInterval` `C`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateDefendLeader` `V`

### `ControllerMonsterStateDodgeAttack` (Game.dll, 14)

- `Confused` `V`
- `ControllerMonsterStateDodgeAttack`
- `ControllerMonsterStateDodgeAttack`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestMove` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateDodgeAttack` `V`

### `ControllerMonsterStateDying` (Game.dll, 18)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterStateDying`
- `ControllerMonsterStateDying`
- `DropLoot`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RespondsToFear` `VC`
- `StartDyingSkill`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateDying` `V`

### `ControllerMonsterStateEmote` (Game.dll, 9)

- `ControllerMonsterStateEmote`
- `ControllerMonsterStateEmote`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateEmote` `V`

### `ControllerMonsterStateFlee` (Game.dll, 14)

- `Confused` `V`
- `ControllerMonsterStateFlee`
- `ControllerMonsterStateFlee`
- `DoFlee`
- `EndOfPathReached` `V`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `PickFleePoint` `C`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateFlee` `V`

### `ControllerMonsterStateFollowLeader` (Game.dll, 23)

- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateFollowLeader`
- `ControllerMonsterStateFollowLeader`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LostSlot` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateFollowLeader` `V`

### `ControllerMonsterStateGettingUp` (Game.dll, 17)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterStateGettingUp`
- `ControllerMonsterStateGettingUp`
- `EndGettingUp` `V`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateGettingUp` `V`

### `ControllerMonsterStateHidden` (Game.dll, 9)

- `ControllerMonsterStateHidden`
- `ControllerMonsterStateHidden`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateHidden` `V`

### `ControllerMonsterStateIdle` (Game.dll, 23)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateIdle`
- `ControllerMonsterStateIdle`
- `EnemyFound` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestConversation` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `StartedRoaming`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateIdle` `V`

### `ControllerMonsterStateImmobile` (Game.dll, 10)

- `ControllerMonsterStateImmobile`
- `ControllerMonsterStateImmobile`
- `EndImmobilize` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateImmobile` `V`

### `ControllerMonsterStateJumpAttack` (Game.dll, 21)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `Confused` `V`
- `ControllerMonsterStateJumpAttack`
- `ControllerMonsterStateJumpAttack`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestMove` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateJumpAttack` `V`

### `ControllerMonsterStateKnockedDown` (Game.dll, 16)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterStateKnockedDown`
- `ControllerMonsterStateKnockedDown`
- `EndKnockdown` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateKnockedDown` `V`

### `ControllerMonsterStateMove` (Game.dll, 14)

- `Confused` `V`
- `ControllerMonsterStateMove`
- `ControllerMonsterStateMove`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestMove` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateMove` `V`

### `ControllerMonsterStateNavigateObstacle` (Game.dll, 11)

- `Confused` `V`
- `ControllerMonsterStateNavigateObstacle`
- `ControllerMonsterStateNavigateObstacle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateNavigateObstacle` `V`

### `ControllerMonsterStatePanic` (Game.dll, 13)

- `Confused` `V`
- `ControllerMonsterStatePanic`
- `ControllerMonsterStatePanic`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RunAway`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStatePanic` `V`

### `ControllerMonsterStateParalyze` (Game.dll, 9)

- `ControllerMonsterStateParalyze`
- `ControllerMonsterStateParalyze`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateParalyze` `V`

### `ControllerMonsterStatePatrol` (Game.dll, 26)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStatePatrol`
- `ControllerMonsterStatePatrol`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LowHealth` `V`
- `MoveToCurrentPatrolPoint`
- `MoveToNextPatrolPoint`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `UsePathPositionAsHome` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStatePatrol` `V`

### `ControllerMonsterStatePursue` (Game.dll, 24)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStatePursue`
- `ControllerMonsterStatePursue`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LostSlot` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStatePursue` `V`

### `ControllerMonsterStateQuestMove` (Game.dll, 24)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateQuestMove`
- `ControllerMonsterStateQuestMove`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `RespondsToFear` `VC`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateQuestMove` `V`

### `ControllerMonsterStateQuestPlayAnimation` (Game.dll, 9)

- `ControllerMonsterStateQuestPlayAnimation`
- `ControllerMonsterStateQuestPlayAnimation`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `RespondsToFear` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateQuestPlayAnimation` `V`

### `ControllerMonsterStateQuestUseSkill` (Game.dll, 9)

- `ControllerMonsterStateQuestUseSkill`
- `ControllerMonsterStateQuestUseSkill`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `RespondsToFear` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateQuestUseSkill` `V`

### `ControllerMonsterStateQuestWalk` (Game.dll, 7)

- `ControllerMonsterStateQuestWalk`
- `ControllerMonsterStateQuestWalk`
- `OnBegin` `V`
- `RespondsToFear` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateQuestWalk` `V`

### `ControllerMonsterStateRepositionForAttack` (Game.dll, 24)

- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateRepositionForAttack`
- `ControllerMonsterStateRepositionForAttack`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LostSlot` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `ProjectileResultCallback` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateRepositionForAttack` `V`

### `ControllerMonsterStateReturn` (Game.dll, 24)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateReturn`
- `ControllerMonsterStateReturn`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `ShouldWalk` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateReturn` `V`

### `ControllerMonsterStateReturnFast` (Game.dll, 6)

- `ControllerMonsterStateReturnFast`
- `ControllerMonsterStateReturnFast`
- `ShouldWalk` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateReturnFast` `V`

### `ControllerMonsterStateRoam` (Game.dll, 24)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateRoam`
- `ControllerMonsterStateRoam`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `StartRoaming`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateRoam` `V`

### `ControllerMonsterStateScared` (Game.dll, 12)

- `ControllerMonsterStateScared`
- `ControllerMonsterStateScared`
- `EndOfPathReached` `V`
- `Hide`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `PickRunToPoint`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateScared` `V`

### `ControllerMonsterStateSleep` (Game.dll, 10)

- `ControllerMonsterStateSleep`
- `ControllerMonsterStateSleep`
- `EndSleep` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateSleep` `V`

### `ControllerMonsterStateStartup` (Game.dll, 18)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerMonsterStateStartup`
- `ControllerMonsterStateStartup`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `UseInitialSkillIfSet`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateStartup` `V`

### `ControllerMonsterStateStunned` (Game.dll, 10)

- `ControllerMonsterStateStunned`
- `ControllerMonsterStateStunned`
- `EndStun` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateStunned` `V`

### `ControllerMonsterStateTakeHit` (Game.dll, 10)

- `ControllerMonsterStateTakeHit`
- `ControllerMonsterStateTakeHit`
- `EndTakeHit` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateTakeHit` `V`

### `ControllerMonsterStateTrapped` (Game.dll, 11)

- `ControllerMonsterStateTrapped`
- `ControllerMonsterStateTrapped`
- `EndTrap` `V`
- `EnemyFound` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateTrapped` `V`

### `ControllerMonsterStateUseSkillOnAlly` (Game.dll, 9)

- `ControllerMonsterStateUseSkillOnAlly`
- `ControllerMonsterStateUseSkillOnAlly`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateUseSkillOnAlly` `V`

### `ControllerMonsterStateUseSkillOnPoint` (Game.dll, 9)

- `ControllerMonsterStateUseSkillOnPoint`
- `ControllerMonsterStateUseSkillOnPoint`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateUseSkillOnPoint` `V`

### `ControllerMonsterStateWaitToAttack` (Game.dll, 26)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateWaitToAttack`
- `ControllerMonsterStateWaitToAttack`
- `EmoteOrRoam`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `HandleEvent` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- `StartRoaming`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateWaitToAttack` `V`

### `ControllerMonsterStateWander` (Game.dll, 24)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `CheckLeaderAndWanderDistance`
- `ClosestEnemyFound` `V`
- `Confused` `V`
- `ControllerMonsterStateWander`
- `ControllerMonsterStateWander`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `LowHealth` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ProjectileNotification` `V`
- `RequestAttack` `V`
- `RequestMove` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateWander` `V`

### `ControllerMonsterStateWanderPause` (Game.dll, 8)

- `ControllerMonsterStateWanderPause`
- `ControllerMonsterStateWanderPause`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterStateWanderPause` `V`

### `ControllerMonsterSynergy` (Game.dll, 25)

- `AddComrade`
- `ControllerMonsterSynergy`
- `ControllerMonsterSynergy`
- `DebugRender` `V`
- `Disband`
- `GetMode` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetSuperSkill` `C`
- `GetTeam` `C`
- `InitialUpdate` `V`
- `Load` `V`
- `PickTeam`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `ResetTimer`
- `SetIdle`
- `SetLeader`
- `SetSlave`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `m_sTeamPool` `S`
- `~ControllerMonsterSynergy` `V`

### `ControllerMonsterSynergyStateDying` (Game.dll, 8)

- `ControllerMonsterSynergyStateDying`
- `ControllerMonsterSynergyStateDying`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterSynergyStateDying` `V`

### `ControllerMonsterSynergyStateSuperAttack` (Game.dll, 9)

- `ControllerMonsterSynergyStateSuperAttack`
- `ControllerMonsterSynergyStateSuperAttack`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerMonsterSynergyStateSuperAttack` `V`

### `ControllerNpc2` (Game.dll, 31)

- `ChatCommandUseSkill` `V`
- `ControllerNpc2`
- `ControllerNpc2`
- `DebugRender` `V`
- `GetCurrentWanderPoint` `C`
- `GetIdleTimeMax` `C`
- `GetIdleTimeMin` `C`
- `GetLoopIdle` `C`
- `GetPlayersInRadius` `C`
- `GetRTTIClassInfo` `VC`
- `GetSocialTarget`
- `GetStaticClassInfo` `S`
- `GetWanderPoints` `C`
- `InitializeStates` `V`
- `IsDebugRenderClicked` `C`
- `Load` `V`
- `OnDebugRenderClick`
- `PostParentAttached` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RegisterTemporaryStates` `V`
- `SetCurrentWanderPoint`
- `Unloading`
- ``vftable'`
- `classInfo` `S`
- `~ControllerNpc2` `V`

### `ControllerNpcAnimalStateFollowHerd` (Game.dll, 10)

- `ControllerNpcAnimalStateFollowHerd`
- `ControllerNpcAnimalStateFollowHerd`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `_MoveToPersonalHerdLocation`
- ``vftable'`
- `operator=`
- `~ControllerNpcAnimalStateFollowHerd` `V`

### `ControllerNpcAnimalStateWiggle` (Game.dll, 9)

- `ControllerNpcAnimalStateWiggle`
- `ControllerNpcAnimalStateWiggle`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcAnimalStateWiggle` `V`

### `ControllerNpcStateAlert` (Game.dll, 12)

- `ControllerNpcStateAlert`
- `ControllerNpcStateAlert`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateAlert` `V`

### `ControllerNpcStateBase` (Game.dll, 11)

- `ControllerNpcStateBase`
- `ControllerNpcStateBase`
- `DefaultChatCommandUseSkill`
- `DefaultQuestCommandMoveResponse`
- `DefaultQuestCommandOrientResponse`
- `DefaultQuestCommandPlayAnimationResponse`
- `DefaultQuestCommandWalkResponse`
- `RequestConversation` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateBase` `V`

### `ControllerNpcStateChat` (Game.dll, 13)

- `ChatCommandUseSkill` `V`
- `ControllerNpcStateChat`
- `ControllerNpcStateChat`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateChat` `V`

### `ControllerNpcStateDirectHerd` (Game.dll, 8)

- `ControllerNpcStateDirectHerd`
- `ControllerNpcStateDirectHerd`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateDirectHerd` `V`

### `ControllerNpcStateIdle` (Game.dll, 14)

- `ChatCommandUseSkill` `V`
- `ControllerNpcStateIdle`
- `ControllerNpcStateIdle`
- `DebugRender` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateIdle` `V`

### `ControllerNpcStateLongIdle` (Game.dll, 8)

- `ControllerNpcStateLongIdle`
- `ControllerNpcStateLongIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateLongIdle` `V`

### `ControllerNpcStateQuestMove` (Game.dll, 14)

- `ControllerNpcStateQuestMove`
- `ControllerNpcStateQuestMove`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateQuestMove` `V`

### `ControllerNpcStateQuestOrient` (Game.dll, 12)

- `ControllerNpcStateQuestOrient`
- `ControllerNpcStateQuestOrient`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateQuestOrient` `V`

### `ControllerNpcStateQuestPlayAnimation` (Game.dll, 13)

- `ControllerNpcStateQuestPlayAnimation`
- `ControllerNpcStateQuestPlayAnimation`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateQuestPlayAnimation` `V`

### `ControllerNpcStateQuestWalk` (Game.dll, 14)

- `ControllerNpcStateQuestWalk`
- `ControllerNpcStateQuestWalk`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateQuestWalk` `V`

### `ControllerNpcStateStartup` (Game.dll, 10)

- `ControllerNpcStateStartup`
- `ControllerNpcStateStartup`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestConversation` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateStartup` `V`

### `ControllerNpcStateUseSkillOnAlly` (Game.dll, 9)

- `ControllerNpcStateUseSkillOnAlly`
- `ControllerNpcStateUseSkillOnAlly`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateUseSkillOnAlly` `V`

### `ControllerNpcStateWander` (Game.dll, 19)

- `ControllerNpcStateWander`
- `ControllerNpcStateWander`
- `EndOfPathReached` `V`
- `FindAttraction`
- `HandleEvent` `V`
- `LostSlot` `V`
- `MoveToCurrentWanderPoint`
- `MoveToNextWanderPoint`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateWander` `V`

### `ControllerNpcStateWatchAttraction` (Game.dll, 13)

- `ControllerNpcStateWatchAttraction`
- `ControllerNpcStateWatchAttraction`
- `LostSlot` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `QuestCommandMove` `V`
- `QuestCommandOrient` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandWalk` `V`
- ``vftable'`
- `operator=`
- `~ControllerNpcStateWatchAttraction` `V`

### `ControllerOrmenos` (Game.dll, 10)

- `ControllerOrmenos`
- `ControllerOrmenos`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RegisterStates` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ControllerOrmenos` `V`

### `ControllerOrmenosStateAttack` (Game.dll, 7)

- `ControllerOrmenosStateAttack`
- `ControllerOrmenosStateAttack`
- `HandleEvent` `V`
- `OnEnd` `V`
- ``vftable'`
- `operator=`
- `~ControllerOrmenosStateAttack` `V`

### `ControllerPet` (Game.dll, 11)

- `ControllerPet`
- `ControllerPet`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `RegisterTemporaryStates` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ControllerPet` `V`

### `ControllerPlayer` (Game.dll, 95)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `BestowToken`
- `CanInterruptMovement` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `ChatWithNPC`
- `CheckAction` `VC`
- `ClearTarget`
- `CompleteRelics` `V`
- `ControllerPlayer`
- `ControllerPlayer`
- `DeathUpdate` `V`
- `EndImmobilize` `V`
- `EndKnockdown` `V`
- `EndSleep` `V`
- `EndStun` `V`
- `EndTakeHit` `V`
- `EndTrap` `V`
- `EvadeAction`
- `FaceTarget`
- `GetAllowWorldEvents` `C`
- `GetAttackDelay` `C`
- `GetCombatAlly` `C`
- `GetCombatEnemy` `C`
- `GetControllerDirection`
- `GetControllerMovementLength`
- `GetInventoryCtrl`
- `GetInventoryCtrl` `C`
- `GetMailboxItem`
- `GetMailboxNpcChat`
- `GetMailboxUserText`
- `GetMailboxUserTextTime`
- `GetMouseRepeatData`
- `GetPrimarySlotOption` `C`
- `GetRTTIClassInfo` `VC`
- `GetSecondarySlotOption` `C`
- `GetStaticClassInfo` `S`
- `GetTargetPoint` `C`
- `GiveArtifactToPlayer`
- `GiveAscendedItemToPlayer`
- `GiveDismantledBonusItemToPlayer`
- `GiveDismantledItemToPlayer`
- `GiveItemToPlayer`
- `GiveRecoveredItemToPlayer`
- `GiveRerollItemToPlayer`
- `GiveSetItemToPlayer`
- `GiveTinkeredItemToPlayer`
- `HandleActionFromJoystick`
- `HandleActionFromMouse`
- `HandlePetAction`
- `InitializeStates` `V`
- `InstantSkillAction`
- `InteractAction` `V`
- `IsCommandRepeated` `C`
- `IsMoveCommand` `C`
- `IsNewMouseCommand` `C`
- `ItemAction` `V`
- `Load` `V`
- `NeedsDeathUpdate` `VC`
- `NpcAction` `V`
- `PopMailboxItem`
- `PopMailboxNpcChat`
- `PopUserText`
- `PostParentAttached` `V`
- `RTTI_new` `S`
- `RegisterStates` `V`
- `ReleasePet` `V`
- `RemoveToken`
- `RespawnMe` `V`
- `SendSkillAction`
- `SetAllowWorldEvents`
- `SetAttackDelay`
- `SetCombatAlly`
- `SetCombatEnemy`
- `SetCommandRepeated`
- `SetControllerDirection`
- `SetControllerMovementLength`
- `SetMouseRepeatData`
- `SetMoveCommand`
- `SetNewMouseCommand`
- `SetPet`
- `SetTargetPoint`
- `SetUserText`
- `StartRespawn`
- `Update` `V`
- `UseItem` `V`
- `UseItemOn` `V`
- ``vftable'`
- `classInfo` `S`
- `~ControllerPlayer` `V`

### `ControllerPlayerState` (Game.dll, 35)

- `ActivateSuperSkill` `V`
- `ControllerPlayerState`
- `ControllerPlayerState`
- `DefaultBeginImmobilizeAction`
- `DefaultBeginKnockdownAction`
- `DefaultBeginSleepAction`
- `DefaultBeginStunAction`
- `DefaultBeginTakeHitAction`
- `DefaultBeginTrapAction`
- `DefaultQuestCommandUseSkillAction`
- `DefaultRequestCompleteRelics`
- `DefaultRequestEvadeAction`
- `DefaultRequestInstantSkillAction`
- `DefaultRequestInteractableAction`
- `DefaultRequestItemAction`
- `DefaultRequestMoveAction`
- `DefaultRequestNpcAction`
- `DefaultRequestReleasePetAction`
- `DefaultRequestRotateAction`
- `DefaultRequestSkillAction`
- `DefaultRequestUseItemAction`
- `DefaultRequestUseItemOnAction`
- `EngageNpc`
- `IsItemPickupAction` `V`
- `MoveAndPickUpItem`
- `MoveAndPickUpItemAlternate`
- `PickupItem`
- `QuestCommandUseSkill` `V`
- `SelectJoystickAction` `V`
- `SelectPrimaryAction` `V`
- `SelectSecondaryAction` `V`
- `Use`
- ``vftable'`
- `operator=`
- `~ControllerPlayerState` `V`

### `ControllerPlayerStateChargeToUseSkill` (Game.dll, 7)

- `ControllerPlayerStateChargeToUseSkill`
- `ControllerPlayerStateChargeToUseSkill`
- `OnBegin` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateChargeToUseSkill` `V`

### `ControllerPlayerStateDying` (Game.dll, 28)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `ControllerPlayerStateDying`
- `ControllerPlayerStateDying`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateDying` `V`

### `ControllerPlayerStateEvade` (Game.dll, 8)

- `ControllerPlayerStateEvade`
- `ControllerPlayerStateEvade`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `PathFailed` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateEvade` `V`

### `ControllerPlayerStateIdle` (Game.dll, 26)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateIdle`
- `ControllerPlayerStateIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateIdle` `V`

### `ControllerPlayerStateImmobilized` (Game.dll, 21)

- `ControllerPlayerStateImmobilized`
- `ControllerPlayerStateImmobilized`
- `EndImmobilize` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateImmobilized` `V`

### `ControllerPlayerStateJumpToUseSkill` (Game.dll, 24)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateJumpToUseSkill`
- `ControllerPlayerStateJumpToUseSkill`
- `EndOfPathReached` `V`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateJumpToUseSkill` `V`

### `ControllerPlayerStateKnockedDown` (Game.dll, 21)

- `ControllerPlayerStateKnockedDown`
- `ControllerPlayerStateKnockedDown`
- `EndKnockdown` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateKnockedDown` `V`

### `ControllerPlayerStateLongIdle` (Game.dll, 27)

- `Attacked` `V`
- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateLongIdle`
- `ControllerPlayerStateLongIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateLongIdle` `V`

### `ControllerPlayerStateMenuIdle` (Game.dll, 26)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateMenuIdle`
- `ControllerPlayerStateMenuIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMenuIdle` `V`

### `ControllerPlayerStateMoveAndUseSkill` (Game.dll, 20)

- `BeginTrap` `V`
- `CanInterruptMovement` `V`
- `ControllerPlayerStateMoveAndUseSkill`
- `ControllerPlayerStateMoveAndUseSkill`
- `EndOfPathReached` `V`
- `HandleEvent` `V`
- `OnUpdate` `V`
- `OnUse` `V`
- `PathFailed` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveAndUseSkill` `V`

### `ControllerPlayerStateMoveTo` (Game.dll, 29)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CanInterruptMovement` `V`
- `ControllerPlayerStateMoveTo`
- `ControllerPlayerStateMoveTo`
- `EndOfPathReached` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveTo` `V`

### `ControllerPlayerStateMoveToActorBase` (Game.dll, 25)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateMoveToActorBase`
- `ControllerPlayerStateMoveToActorBase`
- `OnBegin` `V`
- `OnEnd` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveToActorBase` `V`

### `ControllerPlayerStateMoveToFixedItem` (Game.dll, 10)

- `ControllerPlayerStateMoveToFixedItem`
- `ControllerPlayerStateMoveToFixedItem`
- `EndOfPathReached` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestInteractableAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveToFixedItem` `V`

### `ControllerPlayerStateMoveToItem` (Game.dll, 10)

- `ControllerPlayerStateMoveToItem`
- `ControllerPlayerStateMoveToItem`
- `EndOfPathReached` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestItemAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveToItem` `V`

### `ControllerPlayerStateMoveToNpc` (Game.dll, 9)

- `ControllerPlayerStateMoveToNpc`
- `ControllerPlayerStateMoveToNpc`
- `EndOfPathReached` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestNpcAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveToNpc` `V`

### `ControllerPlayerStateMoveToUseSkill` (Game.dll, 17)

- `BeginTrap` `V`
- `ControllerPlayerStateMoveToUseSkill`
- `ControllerPlayerStateMoveToUseSkill`
- `EndOfPathReached` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateMoveToUseSkill` `V`

### `ControllerPlayerStatePickupItem` (Game.dll, 28)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStatePickupItem`
- `ControllerPlayerStatePickupItem`
- `HandleEvent` `V`
- `IsItemPickupAction` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStatePickupItem` `V`

### `ControllerPlayerStateRespawning` (Game.dll, 29)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `ControllerPlayerStateRespawning`
- `ControllerPlayerStateRespawning`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateRespawning` `V`

### `ControllerPlayerStateSleep` (Game.dll, 21)

- `ControllerPlayerStateSleep`
- `ControllerPlayerStateSleep`
- `EndSleep` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateSleep` `V`

### `ControllerPlayerStateStartup` (Game.dll, 26)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateStartup`
- `ControllerPlayerStateStartup`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateStartup` `V`

### `ControllerPlayerStateStunned` (Game.dll, 21)

- `ControllerPlayerStateStunned`
- `ControllerPlayerStateStunned`
- `EndStun` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateStunned` `V`

### `ControllerPlayerStateTalkToNpc` (Game.dll, 26)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateTalkToNpc`
- `ControllerPlayerStateTalkToNpc`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateTalkToNpc` `V`

### `ControllerPlayerStateTrapped` (Game.dll, 21)

- `ControllerPlayerStateTrapped`
- `ControllerPlayerStateTrapped`
- `EndTrap` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateTrapped` `V`

### `ControllerPlayerStateUseFixedItem` (Game.dll, 27)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateUseFixedItem`
- `ControllerPlayerStateUseFixedItem`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateUseFixedItem` `V`

### `ControllerPlayerStateUseSkill` (Game.dll, 29)

- `BeginImmobilize` `V`
- `BeginKnockdown` `V`
- `BeginSleep` `V`
- `BeginStun` `V`
- `BeginTakeHit` `V`
- `BeginTrap` `V`
- `ControllerPlayerStateUseSkill`
- `ControllerPlayerStateUseSkill`
- `GetSkillUseTolerance` `VC`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `OnUse` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateUseSkill` `V`

### `ControllerPlayerStateUseSkillWhileTrapped` (Game.dll, 22)

- `ControllerPlayerStateUseSkillWhileTrapped`
- `ControllerPlayerStateUseSkillWhileTrapped`
- `EndTrap` `V`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestCompleteRelics` `V`
- `RequestEvadeAction` `V`
- `RequestInstantSkillAction` `V`
- `RequestInteractableAction` `V`
- `RequestItemAction` `V`
- `RequestMoveAction` `V`
- `RequestNpcAction` `V`
- `RequestReleasePet` `V`
- `RequestRotateAction` `V`
- `RequestSkillAction` `V`
- `RequestUseItem` `V`
- `RequestUseItemOn` `V`
- ``vftable'`
- `operator=`
- `~ControllerPlayerStateUseSkillWhileTrapped` `V`

### `ControllerSpiritHostState` (Game.dll, 8)

- `ControllerSpiritHostState`
- `ControllerSpiritHostState`
- `ControllerSpiritHostState`
- `SetAnimated`
- ``vftable'`
- `operator=`
- `operator=`
- `~ControllerSpiritHostState` `V`

### `ControllerSpiritHostStateAnimate` (Game.dll, 8)

- `ControllerSpiritHostStateAnimate`
- `ControllerSpiritHostStateAnimate`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerSpiritHostStateAnimate` `V`

### `ControllerSpiritHostStateStartup` (Game.dll, 9)

- `Attacked` `V`
- `ControllerSpiritHostStateStartup`
- `ControllerSpiritHostStateStartup`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerSpiritHostStateStartup` `V`

### `ControllerSpiritState` (Game.dll, 10)

- `ControllerSpiritState`
- `ControllerSpiritState`
- `ControllerSpiritState`
- `FindNewAnimateTarget`
- `GetAnimateTarget`
- `SetAnimateTarget`
- ``vftable'`
- `operator=`
- `operator=`
- `~ControllerSpiritState` `V`

### `ControllerSpiritStateAttackToAnimate` (Game.dll, 10)

- `ControllerSpiritStateAttackToAnimate`
- `ControllerSpiritStateAttackToAnimate`
- `HandleEvent` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerSpiritStateAttackToAnimate` `V`

### `ControllerSpiritStateIdle` (Game.dll, 8)

- `ControllerSpiritStateIdle`
- `ControllerSpiritStateIdle`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- ``vftable'`
- `operator=`
- `~ControllerSpiritStateIdle` `V`

### `ControllerSpiritStatePursueToAnimate` (Game.dll, 12)

- `ControllerSpiritStatePursueToAnimate`
- `ControllerSpiritStatePursueToAnimate`
- `EndOfPathReached` `V`
- `GetSkillUseTolerance` `VC`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `PathFailed` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerSpiritStatePursueToAnimate` `V`

### `ControllerSpiritStateStartup` (Game.dll, 29)

- `AllyAttacked` `V`
- `AllyDied` `V`
- `AllyNeedsHelp` `V`
- `Attacked` `V`
- `ControllerSpiritStateStartup`
- `ControllerSpiritStateStartup`
- `DoorClosed` `V`
- `DoorOpened` `V`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `HandleEvent` `V`
- `IsAvailableForSidelineConversations` `V`
- `LostSlot` `V`
- `LowHealth` `V`
- `NotifySidelineConversationEnd` `V`
- `NotifySidelineDialogThread` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `RequestAttack` `V`
- `RequestConversation` `V`
- `RequestMove` `V`
- `RequestSidelineConversation` `V`
- `SetMyTeam` `V`
- `ShouldFindClosestEnemy` `VC`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerSpiritStateStartup` `V`

### `ControllerStooge` (Game.dll, 15)

- `CharacterIsDead` `V`
- `CharacterIsDying` `V`
- `ControllerStooge`
- `ControllerStooge`
- `CreateDropMiscItems`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `RTTI_new` `S`
- `SetOnHitSkillId`
- `UnderAttack` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~ControllerStooge` `V`

### `ControllerTerracottaStateAttackMarch` (Game.dll, 12)

- `Attacked` `V`
- `ControllerTerracottaStateAttackMarch`
- `ControllerTerracottaStateAttackMarch`
- `EndOfPathReached` `V`
- `EnemyFound` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerTerracottaStateAttackMarch` `V`

### `ControllerTerracottaStateStartup` (Game.dll, 11)

- `Attacked` `V`
- `ControllerTerracottaStateStartup`
- `ControllerTerracottaStateStartup`
- `EnemyFound` `V`
- `OnBegin` `V`
- `OnEnd` `V`
- `OnUpdate` `V`
- `ShouldFindEnemy` `VC`
- ``vftable'`
- `operator=`
- `~ControllerTerracottaStateStartup` `V`

### `Conversation` (Game.dll, 26)

- `Build`
- `Build`
- `CollectQuestReferences`
- `CollectQuestReferences`
- `Conversation`
- `Conversation`
- `EstablishLinks`
- `GetHash` `C`
- `GetId` `C`
- `GetName` `C`
- `GetOwner` `C`
- `GetPunctuation` `C`
- `GetQuestPunctuation` `C`
- `GetQuestReferences` `C`
- `GetQuestTokens` `C`
- `GetRoot`
- `GetSteps`
- `GetText` `C`
- `HasAvailableSpeech` `C`
- `LogStates` `C`
- `Preload`
- `ReferencesQuests` `C`
- `Update`
- `kVersion` `S`
- `operator=`
- `~Conversation`

### `ConversationResource` (Engine.dll, 10)

- `ConversationResource`
- `Destroy` `V`
- `GetData` `C`
- `GetDataSize` `C`
- `GetHash` `C`
- `GetIsReadyToUse` `C`
- `GetSystemMemoryUsage` `VC`
- `Initialize` `V`
- `InitializeDefault` `V`
- `~ConversationResource` `V`

### `ConversationStep` (Game.dll, 24)

- `ConversationStep`
- `ConversationStep`
- `GetActions` `C`
- `GetConditions` `C`
- `GetFlags` `C`
- `GetLink` `C`
- `GetLinkId` `C`
- `GetLocalizationIndex` `C`
- `GetParent` `C`
- `GetQuestTokens` `C`
- `GetSortOrder` `C`
- `GetSteps` `C`
- `GetType` `C`
- `GetTypeTag` `C`
- `IsAvailable` `C`
- `IsUsed` `C`
- `LogState` `C`
- `PlayVO`
- `Read`
- `SetLink`
- `SetUsed`
- `StopVO`
- `operator=`
- `~ConversationStep`

### `ConversationStore` (Game.dll, 1)

- `AddSpeechToConversations`

### `ConversionAttributeAccumulator` (Game.dll, 9)

- `AddAttribute`
- `Apply`
- `Clear`
- `ConversionAttributeAccumulator`
- `ConversionAttributeAccumulator`
- `GetData`
- `Weigh`
- `operator=`
- `~ConversionAttributeAccumulator`

### `CreateArtifactConfigCmd` (Game.dll, 8)

- `CreateArtifactConfigCmd`
- `CreateArtifactConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeReagents`
- ``vftable'`
- `operator=`
- `~CreateArtifactConfigCmd` `V`

### `CreateArtifactConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateArtifactConfigCmdPacket`
- `CreateArtifactConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateArtifactConfigCmdPacket` `V`

### `CreateEntityPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateEntityPacket`
- `CreateEntityPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateEntityPacket` `V`

### `CreateGravestonePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateGravestonePacket`
- `CreateGravestonePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateGravestonePacket` `V`

### `CreateItemPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateItemPacket`
- `CreateItemPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateItemPacket` `V`

### `CreateItemTeleportPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateItemTeleportPacket`
- `CreateItemTeleportPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateItemTeleportPacket` `V`

### `CreateProjectileConfigCmd` (Game.dll, 7)

- `CreateProjectileConfigCmd`
- `CreateProjectileConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~CreateProjectileConfigCmd` `V`

### `CreateProjectileConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateProjectileConfigCmdPacket`
- `CreateProjectileConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateProjectileConfigCmdPacket` `V`

### `CreateRemoveSkillBuffConfigCmd` (Game.dll, 7)

- `CreateRemoveSkillBuffConfigCmd`
- `CreateRemoveSkillBuffConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~CreateRemoveSkillBuffConfigCmd` `V`

### `CreateRemoveSkillBuffConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `CreateRemoveSkillBuffConfigCmdPacket`
- `CreateRemoveSkillBuffConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~CreateRemoveSkillBuffConfigCmdPacket` `V`

### `CriticalSection` (Engine.dll, 6)

- `CriticalSection`
- `Enter`
- `EnterNoProfile`
- `Exit`
- `TryToEnter`
- `~CriticalSection` `V`

### `CriticalSectionLock` (Engine.dll, 2)

- `CriticalSectionLock`
- `~CriticalSectionLock`

### `CursorHandler` (Game.dll, 51)

- `AllowHandSwap` `VC`
- `CleanupDialog` `V`
- `CreateAndStackIds` `V`
- `CursorHandler`
- `CursorHandler`
- `DeleteThis` `V`
- `Escape` `V`
- `GetEquipmentCtrl`
- `GetGameCursorType` `VC`
- `GetId` `V`
- `GetInventoryCtrl`
- `GetInventoryHighlights` `V`
- `GetOption` `V`
- `GetPlayer`
- `GetPlayerCtrl`
- `GetStashHighlights` `V`
- `GetTransferHighlights` `V`
- `IsEquipConflict` `V`
- `PlayActivationSound` `V`
- `PrimaryEquipActivate` `V`
- `PrimaryInitialize` `V`
- `PrimaryInventoryActivate` `V`
- `PrimaryMarketActivate` `V`
- `PrimaryReagentActivate` `V`
- `PrimaryStashActivate` `V`
- `PrimaryTradeActivate` `V`
- `PrimaryTransferActivate` `V`
- `QuickDropInInventory` `V`
- `QuickDropInReagents` `V`
- `QuickDropInStash` `V`
- `QuickDropInTransfer` `V`
- `ReturnStackedIds` `V`
- `SecondaryEquipActivate` `V`
- `SecondaryInitialize` `V`
- `SecondaryInventoryActivate` `V`
- `SecondaryMarketActivate` `V`
- `SecondaryReagentActivate` `V`
- `SecondaryStashActivate` `V`
- `SecondaryTransferActivate` `V`
- `SetDisplayPosition` `V`
- `SetEquipId` `V`
- `SetId` `V`
- `SetMarketId`
- `SetOption` `V`
- `SetPlayer`
- `SetRelativePosition` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandler` `V`

### `CursorHandlerAttributeReset` (Game.dll, 24)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CleanupDialog` `V`
- `CursorHandlerAttributeReset`
- `CursorHandlerAttributeReset`
- `Escape` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `Render` `VC`
- `SecondaryInitialize` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerAttributeReset` `V`

### `CursorHandlerDevotionReset` (Game.dll, 24)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CleanupDialog` `V`
- `CursorHandlerDevotionReset`
- `CursorHandlerDevotionReset`
- `Escape` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `Render` `VC`
- `SecondaryInitialize` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerDevotionReset` `V`

### `CursorHandlerEnchant` (Game.dll, 40)

- `ActivateWorld` `V`
- `AllowHandSwap` `VC`
- `Cancel` `V`
- `CleanupDialog` `V`
- `ClearHighlight` `V`
- `CreateBitmap` `V`
- `CursorHandlerEnchant`
- `CursorHandlerEnchant`
- `Escape` `V`
- `GetGameCursorType` `VC`
- `GetInventoryHighlights` `V`
- `GetStashHighlights` `V`
- `GetTransferHighlights` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsEquipConflict` `V`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `PrimaryEquipActivate` `V`
- `PrimaryInventoryActivate` `V`
- `PrimaryStashActivate` `V`
- `PrimaryTransferActivate` `V`
- `Render` `VC`
- `SecondaryEquipActivate` `V`
- `SecondaryInventoryActivate` `V`
- `SecondaryStashActivate` `V`
- `SecondaryTransferActivate` `V`
- `SetEquipId` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerEnchant` `V`

### `CursorHandlerFactionBooster` (Game.dll, 24)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CleanupDialog` `V`
- `CursorHandlerFactionBooster`
- `CursorHandlerFactionBooster`
- `Escape` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `Render` `VC`
- `SecondaryInitialize` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerFactionBooster` `V`

### `CursorHandlerHotSlotOption` (Game.dll, 22)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CursorHandlerHotSlotOption`
- `CursorHandlerHotSlotOption`
- `GetInventoryHighlights` `V`
- `GetOption` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `Render` `VC`
- `SetId` `V`
- `SetOption` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerHotSlotOption` `V`

### `CursorHandlerItemMove` (Game.dll, 49)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CleanupDialog` `V`
- `CreateAndStackIds` `V`
- `CursorHandlerItemMove`
- `CursorHandlerItemMove`
- `Escape` `V`
- `GetId` `V`
- `GetInventoryHighlights` `V`
- `GetStashHighlights` `V`
- `GetTransferHighlights` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsEquipConflict` `V`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `PrimaryEquipActivate` `V`
- `PrimaryInitialize` `V`
- `PrimaryInventoryActivate` `V`
- `PrimaryMarketActivate` `V`
- `PrimaryReagentActivate` `V`
- `PrimaryStashActivate` `V`
- `PrimaryTradeActivate` `V`
- `PrimaryTransferActivate` `V`
- `QuickDropInInventory` `V`
- `QuickDropInReagents` `V`
- `QuickDropInStash` `V`
- `QuickDropInTransfer` `V`
- `Render` `VC`
- `SecondaryEquipActivate` `V`
- `SecondaryInitialize` `V`
- `SecondaryInventoryActivate` `V`
- `SecondaryMarketActivate` `V`
- `SecondaryReagentActivate` `V`
- `SecondaryStashActivate` `V`
- `SecondaryTransferActivate` `V`
- `SetEquipId` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerItemMove` `V`

### `CursorHandlerItemUse` (Game.dll, 21)

- `ActivateWorld` `V`
- `Cancel` `V`
- `CursorHandlerItemUse`
- `CursorHandlerItemUse`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `Render` `VC`
- `SecondaryInitialize` `V`
- `SetId` `V`
- `SetSource` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerItemUse` `V`

### `CursorHandlerRelicCombine` (Game.dll, 42)

- `ActivateWorld` `V`
- `AllowHandSwap` `VC`
- `Cancel` `V`
- `CleanupDialog` `V`
- `ClearHighlight` `V`
- `CreateBitmap` `V`
- `CursorHandlerRelicCombine`
- `CursorHandlerRelicCombine`
- `Escape` `V`
- `GetGameCursorType` `VC`
- `GetInventoryHighlights` `V`
- `GetStashHighlights` `V`
- `GetTransferHighlights` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsEquipConflict` `V`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `PrimaryEquipActivate` `V`
- `PrimaryInventoryActivate` `V`
- `PrimaryReagentActivate` `V`
- `PrimaryStashActivate` `V`
- `PrimaryTransferActivate` `V`
- `Render` `VC`
- `SecondaryEquipActivate` `V`
- `SecondaryInventoryActivate` `V`
- `SecondaryReagentActivate` `V`
- `SecondaryStashActivate` `V`
- `SecondaryTransferActivate` `V`
- `SetEquipId` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerRelicCombine` `V`

### `CursorHandlerTransmute` (Game.dll, 35)

- `ActivateWorld` `V`
- `AllowHandSwap` `VC`
- `Cancel` `V`
- `CleanupDialog` `V`
- `CreateBitmap` `V`
- `CursorHandlerTransmute`
- `CursorHandlerTransmute`
- `Escape` `V`
- `GetGameCursorType` `VC`
- `GetInventoryHighlights` `V`
- `IsComplete` `V`
- `IsEnchanterCapable` `VC`
- `IsEquipCapable` `VC`
- `IsEquipConflict` `V`
- `IsHotSlotCapable` `VC`
- `IsInventoryCapable` `VC`
- `IsMarketCapable` `VC`
- `IsShrineCapable` `VC`
- `IsStashCapable` `VC`
- `IsTradeCapable` `VC`
- `IsTransferCapable` `VC`
- `PrimaryEquipActivate` `V`
- `PrimaryInventoryActivate` `V`
- `PrimaryStashActivate` `V`
- `PrimaryTransferActivate` `V`
- `Render` `VC`
- `SecondaryEquipActivate` `V`
- `SecondaryInventoryActivate` `V`
- `SetEquipId` `V`
- `SetId` `V`
- `SetSource` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~CursorHandlerTransmute` `V`

### `CurveData` (Engine.dll, 26)

- `BinaryRead`
- `BinaryWrite` `C`
- `Copy`
- `CurveData`
- `CurveData`
- `CurveData`
- `GetAllocatedSize` `C`
- `GetColor` `C`
- `GetDomain` `C`
- `GetLoop` `C`
- `GetMaxY` `C`
- `GetPointList`
- `GetRange` `C`
- `GetY` `C`
- `GetYFast` `C`
- `SetColor`
- `SetDomain`
- `SetFirstLastY`
- `SetHalfGraph`
- `SetLoop`
- `SetRange`
- `Simplify`
- `UpdateSegmentList`
- `operator=`
- `operator=`
- `~CurveData`

### `DamageAttribute` (Game.dll, 29)

- `ClearDurationModifier` `V`
- `ClearValueModifier` `V`
- `DamageAttribute`
- `DamageAttribute`
- `GetChance` `VC`
- `GetChanceTag` `VC`
- `GetDamageRatio` `VC`
- `GetDisplayTag` `VC`
- `GetDurationModifier` `V`
- `GetDurationV` `V`
- `GetModifierValue` `VC`
- `GetPierceRatio` `VC`
- `GetRangeTag` `C`
- `GetTag` `VC`
- `GetValueModifier` `V`
- `GetValueV` `V`
- `HasGlobalChance` `VC`
- `LoadChance`
- `MergeDamage` `V`
- `SetDamageToLevel` `V`
- `SetHideChanceIndentLine`
- `SetId`
- `SetModifiedMinMax` `V`
- `SetModifiedType` `V`
- `SetNextModifiedMinMax` `V`
- `UseRetaliationTags`
- ``vftable'`
- `operator=`
- `~DamageAttribute` `V`

### `DamageAttributeAbs` (Game.dll, 30)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `DamageAttributeAbs`
- `DamageAttributeAbs`
- `DamageAttributeAbs`
- `GetCostInfo` `VC`
- `GetNextText` `VC`
- `GetRangeNumbers` `VC`
- `GetRangeText` `VC`
- `GetTag` `VC`
- `GetText` `VC`
- `GetValueMax` `VC`
- `GetValueMin` `VC`
- `GetValueV` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDamage` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs` `V`

### `DamageAttributeAbsBase` (Game.dll, 16)

- `AddJitter` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeAbsBase`
- `DamageAttributeAbsBase`
- `DamageAttributeAbsBase`
- `GetCostInfo` `VC`
- `MaxJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase` `V`

### `DamageAttributeAbsBaseElemental` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbsBaseElemental`
- `DamageAttributeAbsBaseElemental`
- `DamageAttributeAbsBaseElemental`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBaseElemental` `V`

### `DamageAttributeAbsBase_Aether` (Game.dll, 14)

- `DamageAttributeAbsBase_Aether`
- `DamageAttributeAbsBase_Aether`
- `DamageAttributeAbsBase_Aether`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Aether` `V`

### `DamageAttributeAbsBase_Chaos` (Game.dll, 14)

- `DamageAttributeAbsBase_Chaos`
- `DamageAttributeAbsBase_Chaos`
- `DamageAttributeAbsBase_Chaos`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Chaos` `V`

### `DamageAttributeAbsBase_Cold` (Game.dll, 14)

- `DamageAttributeAbsBase_Cold`
- `DamageAttributeAbsBase_Cold`
- `DamageAttributeAbsBase_Cold`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Cold` `V`

### `DamageAttributeAbsBase_Fire` (Game.dll, 14)

- `DamageAttributeAbsBase_Fire`
- `DamageAttributeAbsBase_Fire`
- `DamageAttributeAbsBase_Fire`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Fire` `V`

### `DamageAttributeAbsBase_Life` (Game.dll, 14)

- `DamageAttributeAbsBase_Life`
- `DamageAttributeAbsBase_Life`
- `DamageAttributeAbsBase_Life`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Life` `V`

### `DamageAttributeAbsBase_Lightning` (Game.dll, 14)

- `DamageAttributeAbsBase_Lightning`
- `DamageAttributeAbsBase_Lightning`
- `DamageAttributeAbsBase_Lightning`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Lightning` `V`

### `DamageAttributeAbsBase_Poison` (Game.dll, 14)

- `DamageAttributeAbsBase_Poison`
- `DamageAttributeAbsBase_Poison`
- `DamageAttributeAbsBase_Poison`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBase_Poison` `V`

### `DamageAttributeAbsBonus` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeAbsBonus`
- `DamageAttributeAbsBonus`
- `DamageAttributeAbsBonus`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBonus` `V`

### `DamageAttributeAbsBonusElemental` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbsBonusElemental`
- `DamageAttributeAbsBonusElemental`
- `DamageAttributeAbsBonusElemental`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsBonusElemental` `V`

### `DamageAttributeAbsMod` (Game.dll, 28)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `DamageAttributeAbsMod`
- `DamageAttributeAbsMod`
- `DamageAttributeAbsMod`
- `GetCostInfo` `VC`
- `GetModifierValue` `VC`
- `GetNextText` `VC`
- `GetRangeNumbers` `VC`
- `GetRangeText` `VC`
- `GetText` `VC`
- `GetValueModifier` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDamage` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod` `V`

### `DamageAttributeAbsModBase` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeAbsModBase`
- `DamageAttributeAbsModBase`
- `DamageAttributeAbsModBase`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsModBase` `V`

### `DamageAttributeAbsModBonus` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeAbsModBonus`
- `DamageAttributeAbsModBonus`
- `DamageAttributeAbsModBonus`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsModBonus` `V`

### `DamageAttributeAbsMod_Aether` (Game.dll, 13)

- `DamageAttributeAbsMod_Aether`
- `DamageAttributeAbsMod_Aether`
- `DamageAttributeAbsMod_Aether`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Aether` `V`

### `DamageAttributeAbsMod_Bleeding` (Game.dll, 13)

- `DamageAttributeAbsMod_Bleeding`
- `DamageAttributeAbsMod_Bleeding`
- `DamageAttributeAbsMod_Bleeding`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Bleeding` `V`

### `DamageAttributeAbsMod_Chaos` (Game.dll, 13)

- `DamageAttributeAbsMod_Chaos`
- `DamageAttributeAbsMod_Chaos`
- `DamageAttributeAbsMod_Chaos`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Chaos` `V`

### `DamageAttributeAbsMod_Cold` (Game.dll, 13)

- `DamageAttributeAbsMod_Cold`
- `DamageAttributeAbsMod_Cold`
- `DamageAttributeAbsMod_Cold`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Cold` `V`

### `DamageAttributeAbsMod_CritDamageModifier` (Game.dll, 15)

- `AddModifierToAccumulator` `VC`
- `DamageAttributeAbsMod_CritDamageModifier`
- `DamageAttributeAbsMod_CritDamageModifier`
- `DamageAttributeAbsMod_CritDamageModifier`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_CritDamageModifier` `V`

### `DamageAttributeAbsMod_DamageMultiplier` (Game.dll, 14)

- `AddModifierToAccumulator` `VC`
- `DamageAttributeAbsMod_DamageMultiplier`
- `DamageAttributeAbsMod_DamageMultiplier`
- `DamageAttributeAbsMod_DamageMultiplier`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_DamageMultiplier` `V`

### `DamageAttributeAbsMod_Elemental` (Game.dll, 14)

- `AddModifierToAccumulator` `VC`
- `DamageAttributeAbsMod_Elemental`
- `DamageAttributeAbsMod_Elemental`
- `DamageAttributeAbsMod_Elemental`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Elemental` `V`

### `DamageAttributeAbsMod_Fire` (Game.dll, 13)

- `DamageAttributeAbsMod_Fire`
- `DamageAttributeAbsMod_Fire`
- `DamageAttributeAbsMod_Fire`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Fire` `V`

### `DamageAttributeAbsMod_Freeze` (Game.dll, 13)

- `DamageAttributeAbsMod_Freeze`
- `DamageAttributeAbsMod_Freeze`
- `DamageAttributeAbsMod_Freeze`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Freeze` `V`

### `DamageAttributeAbsMod_Knockdown` (Game.dll, 13)

- `DamageAttributeAbsMod_Knockdown`
- `DamageAttributeAbsMod_Knockdown`
- `DamageAttributeAbsMod_Knockdown`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Knockdown` `V`

### `DamageAttributeAbsMod_Life` (Game.dll, 13)

- `DamageAttributeAbsMod_Life`
- `DamageAttributeAbsMod_Life`
- `DamageAttributeAbsMod_Life`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Life` `V`

### `DamageAttributeAbsMod_Lightning` (Game.dll, 13)

- `DamageAttributeAbsMod_Lightning`
- `DamageAttributeAbsMod_Lightning`
- `DamageAttributeAbsMod_Lightning`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Lightning` `V`

### `DamageAttributeAbsMod_ManaBurn` (Game.dll, 13)

- `DamageAttributeAbsMod_ManaBurn`
- `DamageAttributeAbsMod_ManaBurn`
- `DamageAttributeAbsMod_ManaBurn`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_ManaBurn` `V`

### `DamageAttributeAbsMod_Petrify` (Game.dll, 13)

- `DamageAttributeAbsMod_Petrify`
- `DamageAttributeAbsMod_Petrify`
- `DamageAttributeAbsMod_Petrify`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Petrify` `V`

### `DamageAttributeAbsMod_Physical` (Game.dll, 13)

- `DamageAttributeAbsMod_Physical`
- `DamageAttributeAbsMod_Physical`
- `DamageAttributeAbsMod_Physical`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Physical` `V`

### `DamageAttributeAbsMod_Pierce` (Game.dll, 13)

- `DamageAttributeAbsMod_Pierce`
- `DamageAttributeAbsMod_Pierce`
- `DamageAttributeAbsMod_Pierce`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Pierce` `V`

### `DamageAttributeAbsMod_PierceRatio` (Game.dll, 13)

- `DamageAttributeAbsMod_PierceRatio`
- `DamageAttributeAbsMod_PierceRatio`
- `DamageAttributeAbsMod_PierceRatio`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_PierceRatio` `V`

### `DamageAttributeAbsMod_Poison` (Game.dll, 13)

- `DamageAttributeAbsMod_Poison`
- `DamageAttributeAbsMod_Poison`
- `DamageAttributeAbsMod_Poison`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Poison` `V`

### `DamageAttributeAbsMod_Sleep` (Game.dll, 13)

- `DamageAttributeAbsMod_Sleep`
- `DamageAttributeAbsMod_Sleep`
- `DamageAttributeAbsMod_Sleep`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Sleep` `V`

### `DamageAttributeAbsMod_Stun` (Game.dll, 13)

- `DamageAttributeAbsMod_Stun`
- `DamageAttributeAbsMod_Stun`
- `DamageAttributeAbsMod_Stun`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Stun` `V`

### `DamageAttributeAbsMod_TotalDamageModifier` (Game.dll, 14)

- `AddModifierToAccumulator` `VC`
- `DamageAttributeAbsMod_TotalDamageModifier`
- `DamageAttributeAbsMod_TotalDamageModifier`
- `DamageAttributeAbsMod_TotalDamageModifier`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_TotalDamageModifier` `V`

### `DamageAttributeAbsMod_Trap` (Game.dll, 13)

- `DamageAttributeAbsMod_Trap`
- `DamageAttributeAbsMod_Trap`
- `DamageAttributeAbsMod_Trap`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbsMod_Trap` `V`

### `DamageAttributeAbs_Aether` (Game.dll, 14)

- `DamageAttributeAbs_Aether`
- `DamageAttributeAbs_Aether`
- `DamageAttributeAbs_Aether`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Aether` `V`

### `DamageAttributeAbs_BonusPhysical` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_BonusPhysical`
- `DamageAttributeAbs_BonusPhysical`
- `DamageAttributeAbs_BonusPhysical`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_BonusPhysical` `V`

### `DamageAttributeAbs_Chaos` (Game.dll, 14)

- `DamageAttributeAbs_Chaos`
- `DamageAttributeAbs_Chaos`
- `DamageAttributeAbs_Chaos`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Chaos` `V`

### `DamageAttributeAbs_Cold` (Game.dll, 14)

- `DamageAttributeAbs_Cold`
- `DamageAttributeAbs_Cold`
- `DamageAttributeAbs_Cold`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Cold` `V`

### `DamageAttributeAbs_Confusion` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_Confusion`
- `DamageAttributeAbs_Confusion`
- `DamageAttributeAbs_Confusion`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Confusion` `V`

### `DamageAttributeAbs_Convert` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_Convert`
- `DamageAttributeAbs_Convert`
- `DamageAttributeAbs_Convert`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Convert` `V`

### `DamageAttributeAbs_Disruption` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_Disruption`
- `DamageAttributeAbs_Disruption`
- `DamageAttributeAbs_Disruption`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Disruption` `V`

### `DamageAttributeAbs_ElementalDamage` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_ElementalDamage`
- `DamageAttributeAbs_ElementalDamage`
- `DamageAttributeAbs_ElementalDamage`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_ElementalDamage` `V`

### `DamageAttributeAbs_Fear` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_Fear`
- `DamageAttributeAbs_Fear`
- `DamageAttributeAbs_Fear`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Fear` `V`

### `DamageAttributeAbs_Fire` (Game.dll, 14)

- `DamageAttributeAbs_Fire`
- `DamageAttributeAbs_Fire`
- `DamageAttributeAbs_Fire`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Fire` `V`

### `DamageAttributeAbs_Life` (Game.dll, 14)

- `DamageAttributeAbs_Life`
- `DamageAttributeAbs_Life`
- `DamageAttributeAbs_Life`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Life` `V`

### `DamageAttributeAbs_LifeLeech` (Game.dll, 16)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_LifeLeech`
- `DamageAttributeAbs_LifeLeech`
- `DamageAttributeAbs_LifeLeech`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_LifeLeech` `V`

### `DamageAttributeAbs_Lightning` (Game.dll, 14)

- `DamageAttributeAbs_Lightning`
- `DamageAttributeAbs_Lightning`
- `DamageAttributeAbs_Lightning`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Lightning` `V`

### `DamageAttributeAbs_ManaBurn` (Game.dll, 23)

- `AddDamageToAccumulator` `VC`
- `AddToStore` `V`
- `DamageAttributeAbs_ManaBurn`
- `DamageAttributeAbs_ManaBurn`
- `DamageAttributeAbs_ManaBurn`
- `GetDamageRatio` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDamageRatioTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetNextText` `VC`
- `GetText` `VC`
- `GetType` `VC`
- `LoadFromTable` `V`
- `MergeDamage` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_ManaBurn` `V`

### `DamageAttributeAbs_PercentCurrentLife` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_PercentCurrentLife`
- `DamageAttributeAbs_PercentCurrentLife`
- `DamageAttributeAbs_PercentCurrentLife`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_PercentCurrentLife` `V`

### `DamageAttributeAbs_Pierce` (Game.dll, 14)

- `DamageAttributeAbs_Pierce`
- `DamageAttributeAbs_Pierce`
- `DamageAttributeAbs_Pierce`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Pierce` `V`

### `DamageAttributeAbs_Poison` (Game.dll, 14)

- `DamageAttributeAbs_Poison`
- `DamageAttributeAbs_Poison`
- `DamageAttributeAbs_Poison`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Poison` `V`

### `DamageAttributeAbs_Taunt` (Game.dll, 17)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeAbs_Taunt`
- `DamageAttributeAbs_Taunt`
- `DamageAttributeAbs_Taunt`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetNextText` `VC`
- `GetText` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeAbs_Taunt` `V`

### `DamageAttributeDur` (Game.dll, 33)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `DamageAttributeDur`
- `DamageAttributeDur`
- `DamageAttributeDur`
- `GetCostInfo` `VC`
- `GetDurationMax` `VC`
- `GetDurationMin` `VC`
- `GetDurationV` `V`
- `GetNextText` `VC`
- `GetRangeNumbers` `VC`
- `GetRangeText` `VC`
- `GetTag` `VC`
- `GetText` `VC`
- `GetValueMax` `VC`
- `GetValueMin` `VC`
- `GetValueV` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDamage` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur` `V`

### `DamageAttributeDurBase` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeDurBase`
- `DamageAttributeDurBase`
- `DamageAttributeDurBase`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurBase` `V`

### `DamageAttributeDurBaseElemental` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDurBaseElemental`
- `DamageAttributeDurBaseElemental`
- `DamageAttributeDurBaseElemental`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurBaseElemental` `V`

### `DamageAttributeDurBonus` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeDurBonus`
- `DamageAttributeDurBonus`
- `DamageAttributeDurBonus`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurBonus` `V`

### `DamageAttributeDurBonusElemental` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDurBonusElemental`
- `DamageAttributeDurBonusElemental`
- `DamageAttributeDurBonusElemental`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurBonusElemental` `V`

### `DamageAttributeDurFixed` (Game.dll, 16)

- `AddDamageToAccumulator` `VC`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeDurFixed`
- `DamageAttributeDurFixed`
- `DamageAttributeDurFixed`
- `GetNextText` `VC`
- `GetRangeNumbers` `VC`
- `GetRangeText` `VC`
- `GetText` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurFixed` `V`

### `DamageAttributeDurMod` (Game.dll, 33)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `ClearDurationModifier` `V`
- `ClearValueModifier` `V`
- `DamageAttributeDurMod`
- `DamageAttributeDurMod`
- `DamageAttributeDurMod`
- `GetCostInfo` `VC`
- `GetDamageTypeTag` `VC`
- `GetDurationModifier` `V`
- `GetDurationModifierValue` `VC`
- `GetModifierValue` `VC`
- `GetNextText` `VC`
- `GetRangeNumbers` `VC`
- `GetRangeText` `VC`
- `GetText` `VC`
- `GetValueModifier` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDamage` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod` `V`

### `DamageAttributeDurModBase` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeDurModBase`
- `DamageAttributeDurModBase`
- `DamageAttributeDurModBase`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurModBase` `V`

### `DamageAttributeDurModBonus` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeDurModBonus`
- `DamageAttributeDurModBonus`
- `DamageAttributeDurModBonus`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurModBonus` `V`

### `DamageAttributeDurMod_Aether` (Game.dll, 14)

- `DamageAttributeDurMod_Aether`
- `DamageAttributeDurMod_Aether`
- `DamageAttributeDurMod_Aether`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Aether` `V`

### `DamageAttributeDurMod_AttackSpeed` (Game.dll, 14)

- `DamageAttributeDurMod_AttackSpeed`
- `DamageAttributeDurMod_AttackSpeed`
- `DamageAttributeDurMod_AttackSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_AttackSpeed` `V`

### `DamageAttributeDurMod_Bleeding` (Game.dll, 14)

- `DamageAttributeDurMod_Bleeding`
- `DamageAttributeDurMod_Bleeding`
- `DamageAttributeDurMod_Bleeding`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Bleeding` `V`

### `DamageAttributeDurMod_Chaos` (Game.dll, 14)

- `DamageAttributeDurMod_Chaos`
- `DamageAttributeDurMod_Chaos`
- `DamageAttributeDurMod_Chaos`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Chaos` `V`

### `DamageAttributeDurMod_Cold` (Game.dll, 14)

- `DamageAttributeDurMod_Cold`
- `DamageAttributeDurMod_Cold`
- `DamageAttributeDurMod_Cold`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Cold` `V`

### `DamageAttributeDurMod_DamageMultiplier` (Game.dll, 18)

- `AddJitter` `V`
- `DamageAttributeDurMod_DamageMultiplier`
- `DamageAttributeDurMod_DamageMultiplier`
- `DamageAttributeDurMod_DamageMultiplier`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `MaxJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_DamageMultiplier` `V`

### `DamageAttributeDurMod_DefensiveAbility` (Game.dll, 14)

- `DamageAttributeDurMod_DefensiveAbility`
- `DamageAttributeDurMod_DefensiveAbility`
- `DamageAttributeDurMod_DefensiveAbility`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_DefensiveAbility` `V`

### `DamageAttributeDurMod_Fire` (Game.dll, 14)

- `DamageAttributeDurMod_Fire`
- `DamageAttributeDurMod_Fire`
- `DamageAttributeDurMod_Fire`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Fire` `V`

### `DamageAttributeDurMod_Life` (Game.dll, 14)

- `DamageAttributeDurMod_Life`
- `DamageAttributeDurMod_Life`
- `DamageAttributeDurMod_Life`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Life` `V`

### `DamageAttributeDurMod_LifeLeach` (Game.dll, 14)

- `DamageAttributeDurMod_LifeLeach`
- `DamageAttributeDurMod_LifeLeach`
- `DamageAttributeDurMod_LifeLeach`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_LifeLeach` `V`

### `DamageAttributeDurMod_Lightning` (Game.dll, 14)

- `DamageAttributeDurMod_Lightning`
- `DamageAttributeDurMod_Lightning`
- `DamageAttributeDurMod_Lightning`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Lightning` `V`

### `DamageAttributeDurMod_ManaLeach` (Game.dll, 14)

- `DamageAttributeDurMod_ManaLeach`
- `DamageAttributeDurMod_ManaLeach`
- `DamageAttributeDurMod_ManaLeach`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_ManaLeach` `V`

### `DamageAttributeDurMod_OffensiveAbility` (Game.dll, 14)

- `DamageAttributeDurMod_OffensiveAbility`
- `DamageAttributeDurMod_OffensiveAbility`
- `DamageAttributeDurMod_OffensiveAbility`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_OffensiveAbility` `V`

### `DamageAttributeDurMod_OffensiveReduction` (Game.dll, 14)

- `DamageAttributeDurMod_OffensiveReduction`
- `DamageAttributeDurMod_OffensiveReduction`
- `DamageAttributeDurMod_OffensiveReduction`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_OffensiveReduction` `V`

### `DamageAttributeDurMod_Physical` (Game.dll, 14)

- `DamageAttributeDurMod_Physical`
- `DamageAttributeDurMod_Physical`
- `DamageAttributeDurMod_Physical`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Physical` `V`

### `DamageAttributeDurMod_Poison` (Game.dll, 15)

- `DamageAttributeDurMod_Poison`
- `DamageAttributeDurMod_Poison`
- `DamageAttributeDurMod_Poison`
- `GetDamageTypeTag` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_Poison` `V`

### `DamageAttributeDurMod_RunSpeed` (Game.dll, 14)

- `DamageAttributeDurMod_RunSpeed`
- `DamageAttributeDurMod_RunSpeed`
- `DamageAttributeDurMod_RunSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_RunSpeed` `V`

### `DamageAttributeDurMod_SpellCastSpeed` (Game.dll, 14)

- `DamageAttributeDurMod_SpellCastSpeed`
- `DamageAttributeDurMod_SpellCastSpeed`
- `DamageAttributeDurMod_SpellCastSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDurMod_SpellCastSpeed` `V`

### `DamageAttributeDur_Aether` (Game.dll, 16)

- `DamageAttributeDur_Aether`
- `DamageAttributeDur_Aether`
- `DamageAttributeDur_Aether`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Aether` `V`

### `DamageAttributeDur_AttackSpeed` (Game.dll, 16)

- `DamageAttributeDur_AttackSpeed`
- `DamageAttributeDur_AttackSpeed`
- `DamageAttributeDur_AttackSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_AttackSpeed` `V`

### `DamageAttributeDur_Bleeding` (Game.dll, 16)

- `DamageAttributeDur_Bleeding`
- `DamageAttributeDur_Bleeding`
- `DamageAttributeDur_Bleeding`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Bleeding` `V`

### `DamageAttributeDur_Chaos` (Game.dll, 16)

- `DamageAttributeDur_Chaos`
- `DamageAttributeDur_Chaos`
- `DamageAttributeDur_Chaos`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Chaos` `V`

### `DamageAttributeDur_Cold` (Game.dll, 16)

- `DamageAttributeDur_Cold`
- `DamageAttributeDur_Cold`
- `DamageAttributeDur_Cold`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Cold` `V`

### `DamageAttributeDur_DamageMultiplier` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_DamageMultiplier`
- `DamageAttributeDur_DamageMultiplier`
- `DamageAttributeDur_DamageMultiplier`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_DamageMultiplier` `V`

### `DamageAttributeDur_DefensiveAbility` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_DefensiveAbility`
- `DamageAttributeDur_DefensiveAbility`
- `DamageAttributeDur_DefensiveAbility`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_DefensiveAbility` `V`

### `DamageAttributeDur_DefensiveReduction` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_DefensiveReduction`
- `DamageAttributeDur_DefensiveReduction`
- `DamageAttributeDur_DefensiveReduction`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_DefensiveReduction` `V`

### `DamageAttributeDur_ElementalReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_ElementalReductionPercent`
- `DamageAttributeDur_ElementalReductionPercent`
- `DamageAttributeDur_ElementalReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_ElementalReductionPercent` `V`

### `DamageAttributeDur_ElementalResistanceReductionAbsolute` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_ElementalResistanceReductionAbsolute`
- `DamageAttributeDur_ElementalResistanceReductionAbsolute`
- `DamageAttributeDur_ElementalResistanceReductionAbsolute`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_ElementalResistanceReductionAbsolute` `V`

### `DamageAttributeDur_ElementalResistanceReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_ElementalResistanceReductionPercent`
- `DamageAttributeDur_ElementalResistanceReductionPercent`
- `DamageAttributeDur_ElementalResistanceReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_ElementalResistanceReductionPercent` `V`

### `DamageAttributeDur_Fire` (Game.dll, 16)

- `DamageAttributeDur_Fire`
- `DamageAttributeDur_Fire`
- `DamageAttributeDur_Fire`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Fire` `V`

### `DamageAttributeDur_Fumble` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_Fumble`
- `DamageAttributeDur_Fumble`
- `DamageAttributeDur_Fumble`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Fumble` `V`

### `DamageAttributeDur_Life` (Game.dll, 16)

- `DamageAttributeDur_Life`
- `DamageAttributeDur_Life`
- `DamageAttributeDur_Life`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Life` `V`

### `DamageAttributeDur_LifeLeach` (Game.dll, 16)

- `DamageAttributeDur_LifeLeach`
- `DamageAttributeDur_LifeLeach`
- `DamageAttributeDur_LifeLeach`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_LifeLeach` `V`

### `DamageAttributeDur_Lightning` (Game.dll, 16)

- `DamageAttributeDur_Lightning`
- `DamageAttributeDur_Lightning`
- `DamageAttributeDur_Lightning`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Lightning` `V`

### `DamageAttributeDur_ManaLeach` (Game.dll, 16)

- `DamageAttributeDur_ManaLeach`
- `DamageAttributeDur_ManaLeach`
- `DamageAttributeDur_ManaLeach`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_ManaLeach` `V`

### `DamageAttributeDur_OffensiveAbility` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_OffensiveAbility`
- `DamageAttributeDur_OffensiveAbility`
- `DamageAttributeDur_OffensiveAbility`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_OffensiveAbility` `V`

### `DamageAttributeDur_OffensiveReduction` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_OffensiveReduction`
- `DamageAttributeDur_OffensiveReduction`
- `DamageAttributeDur_OffensiveReduction`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_OffensiveReduction` `V`

### `DamageAttributeDur_Physical` (Game.dll, 16)

- `DamageAttributeDur_Physical`
- `DamageAttributeDur_Physical`
- `DamageAttributeDur_Physical`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Physical` `V`

### `DamageAttributeDur_PhysicalReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_PhysicalReductionPercent`
- `DamageAttributeDur_PhysicalReductionPercent`
- `DamageAttributeDur_PhysicalReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_PhysicalReductionPercent` `V`

### `DamageAttributeDur_PhysicalResistanceReductionAbsolute` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_PhysicalResistanceReductionAbsolute`
- `DamageAttributeDur_PhysicalResistanceReductionAbsolute`
- `DamageAttributeDur_PhysicalResistanceReductionAbsolute`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_PhysicalResistanceReductionAbsolute` `V`

### `DamageAttributeDur_PhysicalResistanceReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_PhysicalResistanceReductionPercent`
- `DamageAttributeDur_PhysicalResistanceReductionPercent`
- `DamageAttributeDur_PhysicalResistanceReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_PhysicalResistanceReductionPercent` `V`

### `DamageAttributeDur_Poison` (Game.dll, 16)

- `DamageAttributeDur_Poison`
- `DamageAttributeDur_Poison`
- `DamageAttributeDur_Poison`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_Poison` `V`

### `DamageAttributeDur_ProjectileFumble` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_ProjectileFumble`
- `DamageAttributeDur_ProjectileFumble`
- `DamageAttributeDur_ProjectileFumble`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_ProjectileFumble` `V`

### `DamageAttributeDur_RunSpeed` (Game.dll, 16)

- `DamageAttributeDur_RunSpeed`
- `DamageAttributeDur_RunSpeed`
- `DamageAttributeDur_RunSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_RunSpeed` `V`

### `DamageAttributeDur_SpellCastSpeed` (Game.dll, 16)

- `DamageAttributeDur_SpellCastSpeed`
- `DamageAttributeDur_SpellCastSpeed`
- `DamageAttributeDur_SpellCastSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_SpellCastSpeed` `V`

### `DamageAttributeDur_TotalDamageReductionAbsolute` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_TotalDamageReductionAbsolute`
- `DamageAttributeDur_TotalDamageReductionAbsolute`
- `DamageAttributeDur_TotalDamageReductionAbsolute`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_TotalDamageReductionAbsolute` `V`

### `DamageAttributeDur_TotalDamageReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_TotalDamageReductionPercent`
- `DamageAttributeDur_TotalDamageReductionPercent`
- `DamageAttributeDur_TotalDamageReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_TotalDamageReductionPercent` `V`

### `DamageAttributeDur_TotalResistanceReductionAbsolute` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_TotalResistanceReductionAbsolute`
- `DamageAttributeDur_TotalResistanceReductionAbsolute`
- `DamageAttributeDur_TotalResistanceReductionAbsolute`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_TotalResistanceReductionAbsolute` `V`

### `DamageAttributeDur_TotalResistanceReductionPercent` (Game.dll, 18)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_TotalResistanceReductionPercent`
- `DamageAttributeDur_TotalResistanceReductionPercent`
- `DamageAttributeDur_TotalResistanceReductionPercent`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_TotalResistanceReductionPercent` `V`

### `DamageAttributeDur_TotalSpeed` (Game.dll, 17)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeDur_TotalSpeed`
- `DamageAttributeDur_TotalSpeed`
- `DamageAttributeDur_TotalSpeed`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeDur_TotalSpeed` `V`

### `DamageAttributeInfluence` (Game.dll, 8)

- `DamageAttributeInfluence`
- `DamageAttributeInfluence`
- `DamageAttributeInfluence`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeInfluence` `V`

### `DamageAttributeInfluenceHidden` (Game.dll, 11)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttributeInfluenceHidden`
- `DamageAttributeInfluenceHidden`
- `DamageAttributeInfluenceHidden`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeInfluenceHidden` `V`

### `DamageAttributeReflex` (Game.dll, 9)

- `AddDamageToAccumulator` `VC`
- `DamageAttributeReflex`
- `DamageAttributeReflex`
- `DamageAttributeReflex`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex` `V`

### `DamageAttributeReflex_Freeze` (Game.dll, 15)

- `DamageAttributeReflex_Freeze`
- `DamageAttributeReflex_Freeze`
- `DamageAttributeReflex_Freeze`
- `GetChanceTag` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Freeze` `V`

### `DamageAttributeReflex_Knockdown` (Game.dll, 14)

- `DamageAttributeReflex_Knockdown`
- `DamageAttributeReflex_Knockdown`
- `DamageAttributeReflex_Knockdown`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Knockdown` `V`

### `DamageAttributeReflex_Petrify` (Game.dll, 14)

- `DamageAttributeReflex_Petrify`
- `DamageAttributeReflex_Petrify`
- `DamageAttributeReflex_Petrify`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Petrify` `V`

### `DamageAttributeReflex_Sleep` (Game.dll, 14)

- `DamageAttributeReflex_Sleep`
- `DamageAttributeReflex_Sleep`
- `DamageAttributeReflex_Sleep`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Sleep` `V`

### `DamageAttributeReflex_Stun` (Game.dll, 14)

- `DamageAttributeReflex_Stun`
- `DamageAttributeReflex_Stun`
- `DamageAttributeReflex_Stun`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Stun` `V`

### `DamageAttributeReflex_Trap` (Game.dll, 14)

- `DamageAttributeReflex_Trap`
- `DamageAttributeReflex_Trap`
- `DamageAttributeReflex_Trap`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeReflex_Trap` `V`

### `DamageAttributeStore` (Game.dll, 29)

- `AddAttribute` `V`
- `AddDamageToAccumulator` `VC`
- `AddGlobalAndAttribute` `V`
- `AddGlobalXorAttribute` `V`
- `AddModifierToAccumulator` `VC`
- `Clear`
- `CreateGlobalText` `VC`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `DamageAttributeStore`
- `DamageAttributeStore`
- `GetAttributeId`
- `GetAttributes`
- `GetAttributes` `C`
- `GetCostInfo` `VC`
- `GetGlobalChance` `C`
- `GetRandomGen`
- `IsTypePresent` `C`
- `MergePhysical`
- `MergeStore`
- `MergeStoreAtLevel`
- `ProcessScale` `VC`
- `ProcessText` `VC`
- `ScaleAttributes` `V`
- `SetGlobalChance`
- `SetRandomGen`
- ``vftable'`
- `operator=`
- `~DamageAttributeStore` `V`

### `DamageAttributeStore_Equipment` (Game.dll, 8)

- `DamageAttributeStore_Equipment`
- `DamageAttributeStore_Equipment`
- `DamageAttributeStore_Equipment`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeStore_Equipment` `V`

### `DamageAttributeStore_Max` (Game.dll, 8)

- `DamageAttributeStore_Max`
- `DamageAttributeStore_Max`
- `DamageAttributeStore_Max`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeStore_Max` `V`

### `DamageAttributeStore_Min` (Game.dll, 8)

- `DamageAttributeStore_Min`
- `DamageAttributeStore_Min`
- `DamageAttributeStore_Min`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeStore_Min` `V`

### `DamageAttributeStore_Skill` (Game.dll, 8)

- `DamageAttributeStore_Skill`
- `DamageAttributeStore_Skill`
- `DamageAttributeStore_Skill`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttributeStore_Skill` `V`

### `DamageAttribute_BasePhysical` (Game.dll, 29)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `Create_BasePhysical` `S`
- `DamageAttribute_BasePhysical`
- `DamageAttribute_BasePhysical`
- `GetChance` `VC`
- `GetCostInfo` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- ``vftable'`
- `operator=`
- `~DamageAttribute_BasePhysical` `V`

### `DamageAttribute_Physical` (Game.dll, 36)

- `AddDamageToAccumulator` `VC`
- `AddJitter` `V`
- `AddModifierToAccumulator` `VC`
- `AddToStore` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DamageAttribute_Physical`
- `DamageAttribute_Physical`
- `DamageAttribute_Physical`
- `GetCostInfo` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadXorTag` `VC`
- `GetPierceRatio` `VC`
- `GetTag` `VC`
- `GetType` `VC`
- `GetValueMax` `VC`
- `GetValueMin` `VC`
- `GetValueV` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDamage` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `Scale` `V`
- `ScaleAttribute` `V`
- `SetDamageToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageAttribute_Physical` `V`

### `DamageSectorData` (Engine.dll, 8)

- `Copy` `V`
- `DamageSectorData`
- `DamageSectorData`
- `DamageSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~DamageSectorData` `V`

### `DayNightCycle` (Engine.dll, 32)

- `AddLightSetup`
- `Clear`
- `DayNightCycle`
- `GetDayTime` `C`
- `GetIsDay` `C`
- `GetLightDirection` `C`
- `GetLightSetup` `C`
- `GetLighting` `C`
- `GetMoonAzimuth` `C`
- `GetMoonMinAngle` `C`
- `GetNightTime` `C`
- `GetNumLightSetups` `C`
- `GetSunAzimuth` `C`
- `GetSunMinAngle` `C`
- `GetSunRise` `C`
- `GetSunSet` `C`
- `Read`
- `RemoveLightSetup`
- `SetDayTime`
- `SetDefault`
- `SetLightSetupGroundColor`
- `SetLightSetupLightColor`
- `SetLightSetupSkyColor`
- `SetLightSetupTime`
- `SetMoonAzimuth`
- `SetMoonMinAngle`
- `SetNightTime`
- `SetSunAzimuth`
- `SetSunMinAngle`
- `SetSunRise`
- `SetSunSet`
- `Write` `C`

### `DayNightCycleSectorData` (Engine.dll, 4)

- `GetDayNightCycle`
- `GetDayNightCycle` `C`
- `Read`
- `Write` `C`

### `DayNightLight` (Engine.dll, 11)

- `DayNightLight`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~DayNightLight` `V`

### `DeathHandler` (Game.dll, 20)

- `AnimationCallback` `V`
- `DeathHandler`
- `DeathHandler`
- `Execute` `V`
- `ExternalEvent` `V`
- `Finish` `V`
- `IsFinished`
- `IsSpawnHandler` `VC`
- `NeedsObjectId` `VC`
- `PostProcess` `V`
- `PreLoad` `V`
- `Reset` `V`
- `SetObjectId` `V`
- `ShouldSaveState` `VC`
- `StopEffects` `V`
- `StopMeshEffects` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~DeathHandler` `V`

### `DebugMessagePacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `DebugMessagePacket`
- `DebugMessagePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DebugMessagePacket` `V`

### `DebugRenderManager` (Engine.dll, 30)

- `CreateVertexDeclaration`
- `DebugRenderManager`
- `Disable`
- `Enable`
- `Flush`
- `RenderArrow`
- `RenderBox`
- `RenderBox`
- `RenderBox`
- `RenderCircle`
- `RenderCircleXZ`
- `RenderCone`
- `RenderCone`
- `RenderCylinder`
- `RenderFrustum`
- `RenderLine`
- `RenderLines`
- `RenderPlane`
- `RenderSolidBox`
- `RenderSolidBox`
- `RenderSphere`
- `RenderText`
- `RenderText`
- `RenderThickLine`
- `RenderThickLine`
- `RenderTri`
- `SetFont`
- `SetLocalRegion`
- ``vftable'`
- `~DebugRenderManager` `V`

### `DecBaseDexterityConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DecBaseDexterityConfigCmdPacket`
- `DecBaseDexterityConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DecBaseDexterityConfigCmdPacket` `V`

### `DecBaseIntelligenceConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DecBaseIntelligenceConfigCmdPacket`
- `DecBaseIntelligenceConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DecBaseIntelligenceConfigCmdPacket` `V`

### `DecBaseLifeConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DecBaseLifeConfigCmdPacket`
- `DecBaseLifeConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DecBaseLifeConfigCmdPacket` `V`

### `DecBaseManaConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DecBaseManaConfigCmdPacket`
- `DecBaseManaConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DecBaseManaConfigCmdPacket` `V`

### `DecBaseStrengthConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DecBaseStrengthConfigCmdPacket`
- `DecBaseStrengthConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DecBaseStrengthConfigCmdPacket` `V`

### `Decal` (Engine.dll, 40)

- `AddToScene` `V`
- `AddToWorld`
- `CreateGeometry`
- `DebugRender`
- `Decal`
- `GetIntersection` `VC`
- `GetNumRenderPasses` `VC`
- `GetOpacity` `C`
- `GetRTTIClassInfo` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetStaticClassInfo` `S`
- `GetTexture` `VC`
- `InitialUpdate` `V`
- `IsComplete` `C`
- `IsSavedByEditor` `VC`
- `Load` `V`
- `LogInfo` `VC`
- `MapVertex`
- `OnMoveInLevel` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RebuildMesh`
- `RenderPass` `VC`
- `Reset`
- `SetDynamic`
- `SetMaxAngle`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Decal` `V`

### `DecalSet` (Engine.dll, 19)

- `AddDecal`
- `AddToScene`
- `DecalSet`
- `DecalSet`
- `GetNumRenderPasses` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetTexture` `VC`
- `Initialize`
- `LogInfo` `VC`
- `RemoveAllDecals`
- `RenderPass` `VC`
- `Update`
- ``vftable'`
- `kMaxDecals` `S`
- `kMaxTypes` `S`
- `~DecalSet` `V`

### `DecalTypeMgr` (Engine.dll, 4)

- `Get` `S`
- `LoadDecalType`
- `ReferenceDecalType`
- `UnloadDecalType`

### `Decoration` (Game.dll, 22)

- `AnimationCallback` `V`
- `AttachObject`
- `Decoration`
- `GetIsPartOfLevel` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRandomAnimation`
- `GetStaticClassInfo` `S`
- `IncludeInMinimap` `VC`
- `InitialUpdate` `V`
- `IsStatic` `VC`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `SetInvisible` `V`
- `SetVisible` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Decoration` `V`

### `DecrementBaseDexterityConfigCmd` (Game.dll, 7)

- `DecrementBaseDexterityConfigCmd`
- `DecrementBaseDexterityConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DecrementBaseDexterityConfigCmd` `V`

### `DecrementBaseIntelligenceConfigCmd` (Game.dll, 7)

- `DecrementBaseIntelligenceConfigCmd`
- `DecrementBaseIntelligenceConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DecrementBaseIntelligenceConfigCmd` `V`

### `DecrementBaseLifeConfigCmd` (Game.dll, 7)

- `DecrementBaseLifeConfigCmd`
- `DecrementBaseLifeConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DecrementBaseLifeConfigCmd` `V`

### `DecrementBaseManaConfigCmd` (Game.dll, 7)

- `DecrementBaseManaConfigCmd`
- `DecrementBaseManaConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DecrementBaseManaConfigCmd` `V`

### `DecrementBaseStrengthConfigCmd` (Game.dll, 7)

- `DecrementBaseStrengthConfigCmd`
- `DecrementBaseStrengthConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DecrementBaseStrengthConfigCmd` `V`

### `DefaultDeathHandler` (Game.dll, 12)

- `AnimationCallback` `V`
- `DefaultDeathHandler`
- `DefaultDeathHandler`
- `Execute` `V`
- `ExternalEvent` `V`
- `Finish` `V`
- `IsOverideAllowed` `VC`
- `PostProcess` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~DefaultDeathHandler` `V`

### `DefenseAttribute` (Game.dll, 17)

- `DefenseAttribute`
- `DefenseAttribute`
- `GetChance` `VC`
- `GetValueV` `V`
- `Jitter` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MergeDefense` `V`
- `MinJitter` `V`
- `Scale` `V`
- `SetDefenseToLevel` `V`
- `SetModifiedValue` `V`
- `SetNextModifiedValue` `V`
- `SetRegion` `V`
- ``vftable'`
- `operator=`
- `~DefenseAttribute` `V`

### `DefenseAttributeAbs` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs`
- `DefenseAttributeAbs`
- `DefenseAttributeAbs`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs` `V`

### `DefenseAttributeAbsMod` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbsMod`
- `DefenseAttributeAbsMod`
- `DefenseAttributeAbsMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod` `V`

### `DefenseAttributeAbsMod_Absorption` (Game.dll, 8)

- `DefenseAttributeAbsMod_Absorption`
- `DefenseAttributeAbsMod_Absorption`
- `DefenseAttributeAbsMod_Absorption`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Absorption` `V`

### `DefenseAttributeAbsMod_Aether` (Game.dll, 8)

- `DefenseAttributeAbsMod_Aether`
- `DefenseAttributeAbsMod_Aether`
- `DefenseAttributeAbsMod_Aether`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Aether` `V`

### `DefenseAttributeAbsMod_Bleeding` (Game.dll, 8)

- `DefenseAttributeAbsMod_Bleeding`
- `DefenseAttributeAbsMod_Bleeding`
- `DefenseAttributeAbsMod_Bleeding`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Bleeding` `V`

### `DefenseAttributeAbsMod_Chaos` (Game.dll, 8)

- `DefenseAttributeAbsMod_Chaos`
- `DefenseAttributeAbsMod_Chaos`
- `DefenseAttributeAbsMod_Chaos`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Chaos` `V`

### `DefenseAttributeAbsMod_Cold` (Game.dll, 8)

- `DefenseAttributeAbsMod_Cold`
- `DefenseAttributeAbsMod_Cold`
- `DefenseAttributeAbsMod_Cold`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Cold` `V`

### `DefenseAttributeAbsMod_Elemental` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbsMod_Elemental`
- `DefenseAttributeAbsMod_Elemental`
- `DefenseAttributeAbsMod_Elemental`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Elemental` `V`

### `DefenseAttributeAbsMod_Fire` (Game.dll, 8)

- `DefenseAttributeAbsMod_Fire`
- `DefenseAttributeAbsMod_Fire`
- `DefenseAttributeAbsMod_Fire`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Fire` `V`

### `DefenseAttributeAbsMod_Knockdown` (Game.dll, 8)

- `DefenseAttributeAbsMod_Knockdown`
- `DefenseAttributeAbsMod_Knockdown`
- `DefenseAttributeAbsMod_Knockdown`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Knockdown` `V`

### `DefenseAttributeAbsMod_Life` (Game.dll, 8)

- `DefenseAttributeAbsMod_Life`
- `DefenseAttributeAbsMod_Life`
- `DefenseAttributeAbsMod_Life`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Life` `V`

### `DefenseAttributeAbsMod_LifeLeach` (Game.dll, 8)

- `DefenseAttributeAbsMod_LifeLeach`
- `DefenseAttributeAbsMod_LifeLeach`
- `DefenseAttributeAbsMod_LifeLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_LifeLeach` `V`

### `DefenseAttributeAbsMod_Lightning` (Game.dll, 8)

- `DefenseAttributeAbsMod_Lightning`
- `DefenseAttributeAbsMod_Lightning`
- `DefenseAttributeAbsMod_Lightning`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Lightning` `V`

### `DefenseAttributeAbsMod_ManaLeach` (Game.dll, 8)

- `DefenseAttributeAbsMod_ManaLeach`
- `DefenseAttributeAbsMod_ManaLeach`
- `DefenseAttributeAbsMod_ManaLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_ManaLeach` `V`

### `DefenseAttributeAbsMod_Physical` (Game.dll, 8)

- `DefenseAttributeAbsMod_Physical`
- `DefenseAttributeAbsMod_Physical`
- `DefenseAttributeAbsMod_Physical`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Physical` `V`

### `DefenseAttributeAbsMod_Pierce` (Game.dll, 8)

- `DefenseAttributeAbsMod_Pierce`
- `DefenseAttributeAbsMod_Pierce`
- `DefenseAttributeAbsMod_Pierce`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Pierce` `V`

### `DefenseAttributeAbsMod_Poison` (Game.dll, 8)

- `DefenseAttributeAbsMod_Poison`
- `DefenseAttributeAbsMod_Poison`
- `DefenseAttributeAbsMod_Poison`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Poison` `V`

### `DefenseAttributeAbsMod_Protection` (Game.dll, 8)

- `DefenseAttributeAbsMod_Protection`
- `DefenseAttributeAbsMod_Protection`
- `DefenseAttributeAbsMod_Protection`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Protection` `V`

### `DefenseAttributeAbsMod_Sleep` (Game.dll, 8)

- `DefenseAttributeAbsMod_Sleep`
- `DefenseAttributeAbsMod_Sleep`
- `DefenseAttributeAbsMod_Sleep`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Sleep` `V`

### `DefenseAttributeAbsMod_Stun` (Game.dll, 8)

- `DefenseAttributeAbsMod_Stun`
- `DefenseAttributeAbsMod_Stun`
- `DefenseAttributeAbsMod_Stun`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsMod_Stun` `V`

### `DefenseAttributeAbsRestricted` (Game.dll, 10)

- `AddToAccumulator` `VC`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `DefenseAttributeAbsRestricted`
- `DefenseAttributeAbsRestricted`
- `DefenseAttributeAbsRestricted`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbsRestricted` `V`

### `DefenseAttributeAbs_Aether` (Game.dll, 8)

- `DefenseAttributeAbs_Aether`
- `DefenseAttributeAbs_Aether`
- `DefenseAttributeAbs_Aether`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Aether` `V`

### `DefenseAttributeAbs_AllResistance` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs_AllResistance`
- `DefenseAttributeAbs_AllResistance`
- `DefenseAttributeAbs_AllResistance`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_AllResistance` `V`

### `DefenseAttributeAbs_Bleeding` (Game.dll, 8)

- `DefenseAttributeAbs_Bleeding`
- `DefenseAttributeAbs_Bleeding`
- `DefenseAttributeAbs_Bleeding`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Bleeding` `V`

### `DefenseAttributeAbs_Chaos` (Game.dll, 8)

- `DefenseAttributeAbs_Chaos`
- `DefenseAttributeAbs_Chaos`
- `DefenseAttributeAbs_Chaos`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Chaos` `V`

### `DefenseAttributeAbs_Cold` (Game.dll, 8)

- `DefenseAttributeAbs_Cold`
- `DefenseAttributeAbs_Cold`
- `DefenseAttributeAbs_Cold`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Cold` `V`

### `DefenseAttributeAbs_Confusion` (Game.dll, 8)

- `DefenseAttributeAbs_Confusion`
- `DefenseAttributeAbs_Confusion`
- `DefenseAttributeAbs_Confusion`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Confusion` `V`

### `DefenseAttributeAbs_Convert` (Game.dll, 8)

- `DefenseAttributeAbs_Convert`
- `DefenseAttributeAbs_Convert`
- `DefenseAttributeAbs_Convert`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Convert` `V`

### `DefenseAttributeAbs_CrowdControl` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs_CrowdControl`
- `DefenseAttributeAbs_CrowdControl`
- `DefenseAttributeAbs_CrowdControl`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_CrowdControl` `V`

### `DefenseAttributeAbs_Disruption` (Game.dll, 8)

- `DefenseAttributeAbs_Disruption`
- `DefenseAttributeAbs_Disruption`
- `DefenseAttributeAbs_Disruption`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Disruption` `V`

### `DefenseAttributeAbs_ElementalResistance` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs_ElementalResistance`
- `DefenseAttributeAbs_ElementalResistance`
- `DefenseAttributeAbs_ElementalResistance`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_ElementalResistance` `V`

### `DefenseAttributeAbs_Fear` (Game.dll, 8)

- `DefenseAttributeAbs_Fear`
- `DefenseAttributeAbs_Fear`
- `DefenseAttributeAbs_Fear`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Fear` `V`

### `DefenseAttributeAbs_Fire` (Game.dll, 8)

- `DefenseAttributeAbs_Fire`
- `DefenseAttributeAbs_Fire`
- `DefenseAttributeAbs_Fire`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Fire` `V`

### `DefenseAttributeAbs_Freeze` (Game.dll, 8)

- `DefenseAttributeAbs_Freeze`
- `DefenseAttributeAbs_Freeze`
- `DefenseAttributeAbs_Freeze`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Freeze` `V`

### `DefenseAttributeAbs_Knockdown` (Game.dll, 8)

- `DefenseAttributeAbs_Knockdown`
- `DefenseAttributeAbs_Knockdown`
- `DefenseAttributeAbs_Knockdown`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Knockdown` `V`

### `DefenseAttributeAbs_Life` (Game.dll, 8)

- `DefenseAttributeAbs_Life`
- `DefenseAttributeAbs_Life`
- `DefenseAttributeAbs_Life`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Life` `V`

### `DefenseAttributeAbs_LifeLeach` (Game.dll, 8)

- `DefenseAttributeAbs_LifeLeach`
- `DefenseAttributeAbs_LifeLeach`
- `DefenseAttributeAbs_LifeLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_LifeLeach` `V`

### `DefenseAttributeAbs_Lightning` (Game.dll, 8)

- `DefenseAttributeAbs_Lightning`
- `DefenseAttributeAbs_Lightning`
- `DefenseAttributeAbs_Lightning`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Lightning` `V`

### `DefenseAttributeAbs_ManaBurn` (Game.dll, 12)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DefenseAttributeAbs_ManaBurn`
- `DefenseAttributeAbs_ManaBurn`
- `DefenseAttributeAbs_ManaBurn`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_ManaBurn` `V`

### `DefenseAttributeAbs_ManaBurnRatio` (Game.dll, 12)

- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DefenseAttributeAbs_ManaBurnRatio`
- `DefenseAttributeAbs_ManaBurnRatio`
- `DefenseAttributeAbs_ManaBurnRatio`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_ManaBurnRatio` `V`

### `DefenseAttributeAbs_ManaLeach` (Game.dll, 8)

- `DefenseAttributeAbs_ManaLeach`
- `DefenseAttributeAbs_ManaLeach`
- `DefenseAttributeAbs_ManaLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_ManaLeach` `V`

### `DefenseAttributeAbs_PercentCurrentLife` (Game.dll, 8)

- `DefenseAttributeAbs_PercentCurrentLife`
- `DefenseAttributeAbs_PercentCurrentLife`
- `DefenseAttributeAbs_PercentCurrentLife`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_PercentCurrentLife` `V`

### `DefenseAttributeAbs_PercentReflectionResistance` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs_PercentReflectionResistance`
- `DefenseAttributeAbs_PercentReflectionResistance`
- `DefenseAttributeAbs_PercentReflectionResistance`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_PercentReflectionResistance` `V`

### `DefenseAttributeAbs_Petrify` (Game.dll, 8)

- `DefenseAttributeAbs_Petrify`
- `DefenseAttributeAbs_Petrify`
- `DefenseAttributeAbs_Petrify`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Petrify` `V`

### `DefenseAttributeAbs_Physical` (Game.dll, 8)

- `DefenseAttributeAbs_Physical`
- `DefenseAttributeAbs_Physical`
- `DefenseAttributeAbs_Physical`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Physical` `V`

### `DefenseAttributeAbs_Pierce` (Game.dll, 8)

- `DefenseAttributeAbs_Pierce`
- `DefenseAttributeAbs_Pierce`
- `DefenseAttributeAbs_Pierce`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Pierce` `V`

### `DefenseAttributeAbs_Poison` (Game.dll, 8)

- `DefenseAttributeAbs_Poison`
- `DefenseAttributeAbs_Poison`
- `DefenseAttributeAbs_Poison`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Poison` `V`

### `DefenseAttributeAbs_Sleep` (Game.dll, 8)

- `DefenseAttributeAbs_Sleep`
- `DefenseAttributeAbs_Sleep`
- `DefenseAttributeAbs_Sleep`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Sleep` `V`

### `DefenseAttributeAbs_Stun` (Game.dll, 8)

- `DefenseAttributeAbs_Stun`
- `DefenseAttributeAbs_Stun`
- `DefenseAttributeAbs_Stun`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Stun` `V`

### `DefenseAttributeAbs_Taunt` (Game.dll, 8)

- `DefenseAttributeAbs_Taunt`
- `DefenseAttributeAbs_Taunt`
- `DefenseAttributeAbs_Taunt`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Taunt` `V`

### `DefenseAttributeAbs_TotalSpeed` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeAbs_TotalSpeed`
- `DefenseAttributeAbs_TotalSpeed`
- `DefenseAttributeAbs_TotalSpeed`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_TotalSpeed` `V`

### `DefenseAttributeAbs_Trap` (Game.dll, 8)

- `DefenseAttributeAbs_Trap`
- `DefenseAttributeAbs_Trap`
- `DefenseAttributeAbs_Trap`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeAbs_Trap` `V`

### `DefenseAttributeDefenseCap` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeDefenseCap`
- `DefenseAttributeDefenseCap`
- `DefenseAttributeDefenseCap`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap` `V`

### `DefenseAttributeDefenseCap_Aether` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Aether`
- `DefenseAttributeDefenseCap_Aether`
- `DefenseAttributeDefenseCap_Aether`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Aether` `V`

### `DefenseAttributeDefenseCap_All` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeDefenseCap_All`
- `DefenseAttributeDefenseCap_All`
- `DefenseAttributeDefenseCap_All`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_All` `V`

### `DefenseAttributeDefenseCap_Bleeding` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Bleeding`
- `DefenseAttributeDefenseCap_Bleeding`
- `DefenseAttributeDefenseCap_Bleeding`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Bleeding` `V`

### `DefenseAttributeDefenseCap_Chaos` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Chaos`
- `DefenseAttributeDefenseCap_Chaos`
- `DefenseAttributeDefenseCap_Chaos`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Chaos` `V`

### `DefenseAttributeDefenseCap_Cold` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Cold`
- `DefenseAttributeDefenseCap_Cold`
- `DefenseAttributeDefenseCap_Cold`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Cold` `V`

### `DefenseAttributeDefenseCap_CrowdControl` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeDefenseCap_CrowdControl`
- `DefenseAttributeDefenseCap_CrowdControl`
- `DefenseAttributeDefenseCap_CrowdControl`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_CrowdControl` `V`

### `DefenseAttributeDefenseCap_Disruption` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Disruption`
- `DefenseAttributeDefenseCap_Disruption`
- `DefenseAttributeDefenseCap_Disruption`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Disruption` `V`

### `DefenseAttributeDefenseCap_Fire` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Fire`
- `DefenseAttributeDefenseCap_Fire`
- `DefenseAttributeDefenseCap_Fire`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Fire` `V`

### `DefenseAttributeDefenseCap_Freeze` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Freeze`
- `DefenseAttributeDefenseCap_Freeze`
- `DefenseAttributeDefenseCap_Freeze`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Freeze` `V`

### `DefenseAttributeDefenseCap_Life` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Life`
- `DefenseAttributeDefenseCap_Life`
- `DefenseAttributeDefenseCap_Life`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Life` `V`

### `DefenseAttributeDefenseCap_LifeLeach` (Game.dll, 8)

- `DefenseAttributeDefenseCap_LifeLeach`
- `DefenseAttributeDefenseCap_LifeLeach`
- `DefenseAttributeDefenseCap_LifeLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_LifeLeach` `V`

### `DefenseAttributeDefenseCap_Lightning` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Lightning`
- `DefenseAttributeDefenseCap_Lightning`
- `DefenseAttributeDefenseCap_Lightning`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Lightning` `V`

### `DefenseAttributeDefenseCap_ManaLeach` (Game.dll, 8)

- `DefenseAttributeDefenseCap_ManaLeach`
- `DefenseAttributeDefenseCap_ManaLeach`
- `DefenseAttributeDefenseCap_ManaLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_ManaLeach` `V`

### `DefenseAttributeDefenseCap_Petrify` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Petrify`
- `DefenseAttributeDefenseCap_Petrify`
- `DefenseAttributeDefenseCap_Petrify`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Petrify` `V`

### `DefenseAttributeDefenseCap_Physical` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Physical`
- `DefenseAttributeDefenseCap_Physical`
- `DefenseAttributeDefenseCap_Physical`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Physical` `V`

### `DefenseAttributeDefenseCap_Pierce` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Pierce`
- `DefenseAttributeDefenseCap_Pierce`
- `DefenseAttributeDefenseCap_Pierce`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Pierce` `V`

### `DefenseAttributeDefenseCap_Poison` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Poison`
- `DefenseAttributeDefenseCap_Poison`
- `DefenseAttributeDefenseCap_Poison`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Poison` `V`

### `DefenseAttributeDefenseCap_Sleep` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Sleep`
- `DefenseAttributeDefenseCap_Sleep`
- `DefenseAttributeDefenseCap_Sleep`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Sleep` `V`

### `DefenseAttributeDefenseCap_Stun` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Stun`
- `DefenseAttributeDefenseCap_Stun`
- `DefenseAttributeDefenseCap_Stun`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Stun` `V`

### `DefenseAttributeDefenseCap_Trap` (Game.dll, 8)

- `DefenseAttributeDefenseCap_Trap`
- `DefenseAttributeDefenseCap_Trap`
- `DefenseAttributeDefenseCap_Trap`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDefenseCap_Trap` `V`

### `DefenseAttributeDur` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeDur`
- `DefenseAttributeDur`
- `DefenseAttributeDur`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur` `V`

### `DefenseAttributeDurMod` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeDurMod`
- `DefenseAttributeDurMod`
- `DefenseAttributeDurMod`
- `GetCostInfo` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod` `V`

### `DefenseAttributeDurMod_Aether` (Game.dll, 8)

- `DefenseAttributeDurMod_Aether`
- `DefenseAttributeDurMod_Aether`
- `DefenseAttributeDurMod_Aether`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Aether` `V`

### `DefenseAttributeDurMod_Bleeding` (Game.dll, 8)

- `DefenseAttributeDurMod_Bleeding`
- `DefenseAttributeDurMod_Bleeding`
- `DefenseAttributeDurMod_Bleeding`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Bleeding` `V`

### `DefenseAttributeDurMod_Chaos` (Game.dll, 8)

- `DefenseAttributeDurMod_Chaos`
- `DefenseAttributeDurMod_Chaos`
- `DefenseAttributeDurMod_Chaos`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Chaos` `V`

### `DefenseAttributeDurMod_Cold` (Game.dll, 8)

- `DefenseAttributeDurMod_Cold`
- `DefenseAttributeDurMod_Cold`
- `DefenseAttributeDurMod_Cold`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Cold` `V`

### `DefenseAttributeDurMod_Fire` (Game.dll, 8)

- `DefenseAttributeDurMod_Fire`
- `DefenseAttributeDurMod_Fire`
- `DefenseAttributeDurMod_Fire`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Fire` `V`

### `DefenseAttributeDurMod_Life` (Game.dll, 8)

- `DefenseAttributeDurMod_Life`
- `DefenseAttributeDurMod_Life`
- `DefenseAttributeDurMod_Life`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Life` `V`

### `DefenseAttributeDurMod_LifeLeach` (Game.dll, 8)

- `DefenseAttributeDurMod_LifeLeach`
- `DefenseAttributeDurMod_LifeLeach`
- `DefenseAttributeDurMod_LifeLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_LifeLeach` `V`

### `DefenseAttributeDurMod_Lightning` (Game.dll, 8)

- `DefenseAttributeDurMod_Lightning`
- `DefenseAttributeDurMod_Lightning`
- `DefenseAttributeDurMod_Lightning`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Lightning` `V`

### `DefenseAttributeDurMod_ManaLeach` (Game.dll, 8)

- `DefenseAttributeDurMod_ManaLeach`
- `DefenseAttributeDurMod_ManaLeach`
- `DefenseAttributeDurMod_ManaLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_ManaLeach` `V`

### `DefenseAttributeDurMod_Physical` (Game.dll, 8)

- `DefenseAttributeDurMod_Physical`
- `DefenseAttributeDurMod_Physical`
- `DefenseAttributeDurMod_Physical`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Physical` `V`

### `DefenseAttributeDurMod_Poison` (Game.dll, 8)

- `DefenseAttributeDurMod_Poison`
- `DefenseAttributeDurMod_Poison`
- `DefenseAttributeDurMod_Poison`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDurMod_Poison` `V`

### `DefenseAttributeDur_Aether` (Game.dll, 8)

- `DefenseAttributeDur_Aether`
- `DefenseAttributeDur_Aether`
- `DefenseAttributeDur_Aether`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Aether` `V`

### `DefenseAttributeDur_Bleeding` (Game.dll, 8)

- `DefenseAttributeDur_Bleeding`
- `DefenseAttributeDur_Bleeding`
- `DefenseAttributeDur_Bleeding`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Bleeding` `V`

### `DefenseAttributeDur_Chaos` (Game.dll, 8)

- `DefenseAttributeDur_Chaos`
- `DefenseAttributeDur_Chaos`
- `DefenseAttributeDur_Chaos`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Chaos` `V`

### `DefenseAttributeDur_Cold` (Game.dll, 8)

- `DefenseAttributeDur_Cold`
- `DefenseAttributeDur_Cold`
- `DefenseAttributeDur_Cold`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Cold` `V`

### `DefenseAttributeDur_Fire` (Game.dll, 8)

- `DefenseAttributeDur_Fire`
- `DefenseAttributeDur_Fire`
- `DefenseAttributeDur_Fire`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Fire` `V`

### `DefenseAttributeDur_Life` (Game.dll, 8)

- `DefenseAttributeDur_Life`
- `DefenseAttributeDur_Life`
- `DefenseAttributeDur_Life`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Life` `V`

### `DefenseAttributeDur_LifeLeach` (Game.dll, 8)

- `DefenseAttributeDur_LifeLeach`
- `DefenseAttributeDur_LifeLeach`
- `DefenseAttributeDur_LifeLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_LifeLeach` `V`

### `DefenseAttributeDur_Lightning` (Game.dll, 8)

- `DefenseAttributeDur_Lightning`
- `DefenseAttributeDur_Lightning`
- `DefenseAttributeDur_Lightning`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Lightning` `V`

### `DefenseAttributeDur_ManaLeach` (Game.dll, 8)

- `DefenseAttributeDur_ManaLeach`
- `DefenseAttributeDur_ManaLeach`
- `DefenseAttributeDur_ManaLeach`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_ManaLeach` `V`

### `DefenseAttributeDur_Physical` (Game.dll, 8)

- `DefenseAttributeDur_Physical`
- `DefenseAttributeDur_Physical`
- `DefenseAttributeDur_Physical`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Physical` `V`

### `DefenseAttributeDur_Poison` (Game.dll, 8)

- `DefenseAttributeDur_Poison`
- `DefenseAttributeDur_Poison`
- `DefenseAttributeDur_Poison`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeDur_Poison` `V`

### `DefenseAttributeMisc_BlockAmountModifier` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeMisc_BlockAmountModifier`
- `DefenseAttributeMisc_BlockAmountModifier`
- `DefenseAttributeMisc_BlockAmountModifier`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeMisc_BlockAmountModifier` `V`

### `DefenseAttributeMisc_BlockModifier` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeMisc_BlockModifier`
- `DefenseAttributeMisc_BlockModifier`
- `DefenseAttributeMisc_BlockModifier`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeMisc_BlockModifier` `V`

### `DefenseAttributeMisc_NegativeDamageMultiplier` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeMisc_NegativeDamageMultiplier`
- `DefenseAttributeMisc_NegativeDamageMultiplier`
- `DefenseAttributeMisc_NegativeDamageMultiplier`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeMisc_NegativeDamageMultiplier` `V`

### `DefenseAttributeMisc_Reflect` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeMisc_Reflect`
- `DefenseAttributeMisc_Reflect`
- `DefenseAttributeMisc_Reflect`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeMisc_Reflect` `V`

### `DefenseAttributeMisc_ReflectModifier` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `DefenseAttributeMisc_ReflectModifier`
- `DefenseAttributeMisc_ReflectModifier`
- `DefenseAttributeMisc_ReflectModifier`
- `GetType` `VC`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeMisc_ReflectModifier` `V`

### `DefenseAttributeStore` (Game.dll, 21)

- `AddAttribute` `V`
- `AddToAccumulator` `VC`
- `Clear`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `DefenseAttributeStore`
- `DefenseAttributeStore`
- `GetAttributes`
- `GetAttributes` `C`
- `GetCostInfo` `C`
- `GetRandomGen`
- `IsTypePresent` `C`
- `MergeStore`
- `MergeStoreAtLevel`
- `ProcessText` `C`
- `ScaleAttributes` `V`
- `SetCombatRegion` `C`
- `SetRandomGen`
- ``vftable'`
- `operator=`
- `~DefenseAttributeStore` `V`

### `DefenseAttributeStore_Character` (Game.dll, 8)

- `DefenseAttributeStore_Character`
- `DefenseAttributeStore_Character`
- `DefenseAttributeStore_Character`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeStore_Character` `V`

### `DefenseAttributeStore_Equipment` (Game.dll, 8)

- `DefenseAttributeStore_Equipment`
- `DefenseAttributeStore_Equipment`
- `DefenseAttributeStore_Equipment`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeStore_Equipment` `V`

### `DefenseAttributeStore_Max` (Game.dll, 8)

- `DefenseAttributeStore_Max`
- `DefenseAttributeStore_Max`
- `DefenseAttributeStore_Max`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeStore_Max` `V`

### `DefenseAttributeStore_Min` (Game.dll, 8)

- `DefenseAttributeStore_Min`
- `DefenseAttributeStore_Min`
- `DefenseAttributeStore_Min`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeStore_Min` `V`

### `DefenseAttributeStore_Skill` (Game.dll, 8)

- `DefenseAttributeStore_Skill`
- `DefenseAttributeStore_Skill`
- `DefenseAttributeStore_Skill`
- `Load` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttributeStore_Skill` `V`

### `DefenseAttribute_BaseProtectionAbsorption` (Game.dll, 29)

- `AddJitter` `V`
- `AddToAccumulator` `VC`
- `AddToStore` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `Create_BaseAbsorptionProtection` `S`
- `Create_BaseAbsorptionProtection_Max` `S`
- `Create_BaseAbsorptionProtection_Min` `S`
- `DefenseAttribute_BaseProtectionAbsorption`
- `DefenseAttribute_BaseProtectionAbsorption`
- `DefenseAttribute_BaseProtectionAbsorption`
- `GetBonusProtection` `C`
- `GetCostInfo` `VC`
- `GetProtection` `C`
- `GetType` `VC`
- `LoadFromTable` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- `SetRegion` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttribute_BaseProtectionAbsorption` `V`

### `DefenseAttribute_SkillProtectionAbsorption` (Game.dll, 9)

- `CreateNextText` `VC`
- `CreateText` `VC`
- `DefenseAttribute_SkillProtectionAbsorption`
- `DefenseAttribute_SkillProtectionAbsorption`
- `DefenseAttribute_SkillProtectionAbsorption`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttribute_SkillProtectionAbsorption` `V`

### `DefenseAttribute_Typical` (Game.dll, 24)

- `AddJitter` `V`
- `AddToStore` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `DefenseAttribute_Typical`
- `DefenseAttribute_Typical`
- `DefenseAttribute_Typical`
- `GetRangeTag` `C`
- `GetValue` `VC`
- `GetValueV` `V`
- `LoadFromTable` `V`
- `MaxJitter` `V`
- `MaxJitter` `V`
- `MergeDefense` `V`
- `MinJitter` `V`
- `MinJitter` `V`
- `ScaleAttribute` `V`
- `SetDefenseToLevel` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~DefenseAttribute_Typical` `V`

### `DestroyGravestonePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DestroyGravestonePacket`
- `DestroyGravestonePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DestroyGravestonePacket` `V`

### `DestroyItemTeleportPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DestroyItemTeleportPacket`
- `DestroyItemTeleportPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DestroyItemTeleportPacket` `V`

### `Destructible` (Game.dll, 59)

- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `BreakApart`
- `BreakModeFromString` `C`
- `CollisionCallback` `V`
- `CreatePathObstacles` `V`
- `DestroyTrails` `V`
- `Destructible`
- `DynamicPathingOccluder` `VC`
- `GeometryBusStop` `V`
- `GeometryBusStop` `V`
- `GetAttackAnimation` `C`
- `GetIntersection` `VC`
- `GetIsPartOfLevel` `VC`
- `GetLife` `C`
- `GetLootDropCoords` `VC`
- `GetLootDropGroup` `VC`
- `GetLootDropRadius` `VC`
- `GetMaxLife` `C`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsBroken` `C`
- `IsStatic` `VC`
- `IsTargetable` `C`
- `Load` `V`
- `OccludesPathing` `VC`
- `OnDestroy` `V`
- `PassabilityFromString`
- `PhysicsPost` `V`
- `PhysicsSetup` `V`
- `PhysicsUpdate` `V`
- `PickStatics`
- `PlayBodyFallSound`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `ReplicateBreakApart`
- `ResolveEquationVariable` `VC`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetBlocked` `V`
- `SetHighlight` `V`
- `SetInvulnerable`
- `SetParentLevel` `V`
- `ShouldCastShadows` `VC`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `TakeAttack` `V`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Destructible` `V`

### `DestructibleHookPack` (Game.dll, 6)

- `DestructibleHookPack`
- `DestructibleHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~DestructibleHookPack` `V`

### `DetachItemConfigCmd` (Game.dll, 7)

- `DetachItemConfigCmd`
- `DetachItemConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~DetachItemConfigCmd` `V`

### `DetachItemConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DetachItemConfigCmdPacket`
- `DetachItemConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DetachItemConfigCmdPacket` `V`

### `DetailMapFilterFunction` (Engine.dll, 7)

- `DetailMapFilterFunction`
- `DetailMapFilterFunction`
- `DetailMapFilterFunction`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `DialogManager` (Game.dll, 14)

- `AddDialog`
- `AddDialog`
- `AddResponse`
- `Clear`
- `DialogManager`
- `DialogManager`
- `GetNumDialog` `C`
- `GetNumResponsesFor` `C`
- `GetResponseFor`
- `GetTopInterestedParty` `C`
- `PeekTopDialog`
- `RemoveTopDialog`
- `operator=`
- `~DialogManager`

### `DieAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `DieAction`
- `DieAction`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- ``vftable'`
- `~DieAction` `V`

### `DiePacket` (Game.dll, 8)

- `Deserialize` `V`
- `DiePacket`
- `DiePacket`
- `GetPacketDescription` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~DiePacket` `V`

### `DifficultyRampUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DifficultyRampUpdatePacket`
- `DifficultyRampUpdatePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DifficultyRampUpdatePacket` `V`

### `DirectoryBrowser` (Widget.dll, 14)

- `Create`
- `Destroy`
- `DirectoryBrowser`
- `EnsureVisible`
- `ForceDirectorySelection`
- `GetSelectedDirectory` `C`
- `GetSelectedItems` `C`
- `GetSource` `C`
- `SetFileSystem`
- `SetMultipleSelection`
- `SetRootDirectory`
- `SetSource`
- `ShowFiles`
- `ShowRecords`

### `DirtyRect` (Engine.dll, 5)

- `DirtyRect`
- `DirtyRect`
- `Overlap` `S`
- `operator=`
- `operator=`

### `DirtyRectSet` (Engine.dll, 12)

- `AddRect`
- `AddRect`
- `Clear`
- `DirtyRectSet`
- `DirtyRectSet`
- `DirtyRectSet`
- `GetBoundingRect` `C`
- `GetNumRects` `C`
- `GetRect` `C`
- `operator=`
- `operator=`
- `~DirtyRectSet`

### `DisconnectClientPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `DisconnectClientPacket`
- `DisconnectClientPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisconnectClientPacket` `V`

### `DisengageAltarPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DisengageAltarPacket`
- `DisengageAltarPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisengageAltarPacket` `V`

### `DisengageContainerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DisengageContainerPacket`
- `DisengageContainerPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisengageContainerPacket` `V`

### `DisengageEndlessShrinePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DisengageEndlessShrinePacket`
- `DisengageEndlessShrinePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisengageEndlessShrinePacket` `V`

### `DisengageNpcPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DisengageNpcPacket`
- `DisengageNpcPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisengageNpcPacket` `V`

### `DisengageShrinePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DisengageShrinePacket`
- `DisengageShrinePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DisengageShrinePacket` `V`

### `Display` (Engine.dll, 9)

- `AddWidget`
- `Display`
- `HandleControllerEvent`
- `HandleJoystickEvent`
- `HandleKeyEvent`
- `HandleMouseEvent`
- `RemoveWidget`
- `Render` `C`
- `Update`

### `DisplayWidget` (Engine.dll, 11)

- `DisplayWidget`
- `DisplayWidget`
- `DisplayWidget`
- `HandleControllerEvent` `V`
- `HandleJoystickEvent` `V`
- `HandleKeyEvent` `V`
- `HandleMouseEvent` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `operator=`

### `DissolveActorDeathHandler` (Game.dll, 13)

- `AnimationCallback` `V`
- `DissolveActorDeathHandler`
- `DissolveActorDeathHandler`
- `Execute` `V`
- `ExternalEvent` `V`
- `Finish` `V`
- `IsOverideAllowed` `VC`
- `PostProcess` `V`
- `StartDissolve` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~DissolveActorDeathHandler` `V`

### `DungeonOpenExitPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DungeonOpenExitPacket`
- `DungeonOpenExitPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DungeonOpenExitPacket` `V`

### `DungeonProgressToClientPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DungeonProgressToClientPacket`
- `DungeonProgressToClientPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DungeonProgressToClientPacket` `V`

### `DungeonProgressToServerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DungeonProgressToServerPacket`
- `DungeonProgressToServerPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DungeonProgressToServerPacket` `V`

### `DungeonSoundPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `DungeonSoundPacket`
- `DungeonSoundPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~DungeonSoundPacket` `V`

### `DurationDamageManager` (Game.dll, 31)

- `AddDamage` `V`
- `AddDamageEffect`
- `AddDamageReplication`
- `AddFixedDamage` `V`
- `AddFixedDamageReplication`
- `AssignParents`
- `BeginNewAttack`
- `CalculateAllocatedMemory` `C`
- `DurationDamageManager`
- `DurationDamageManager`
- `EndAttack`
- `ExecuteDamage`
- `GetDamageData`
- `GetDamageReplica` `C`
- `GetFixedDamage` `VC`
- `GetFixedDamageDuration` `VC`
- `GetFixedDamageReplica` `C`
- `GetSpeedUpdate` `C`
- `ImDead`
- `IsPoisoned` `C`
- `ModifyDuration`
- `ProcessDamage`
- `RemoveAllDamages` `V`
- `SetAttacker`
- `SetDamageReplica`
- `SetFixedDamageReplica`
- `Update` `V`
- `UpdateFxAndInfluence`
- ``vftable'`
- `operator=`
- `~DurationDamageManager` `V`

### `DynamicDoor` (Game.dll, 36)

- `AnimationCallback` `V`
- `ApplyReplicationData` `V`
- `CanInteract` `VC`
- `CanInteractWithEntity` `VC`
- `Close` `V`
- `DynamicDoor`
- `GetOpenCloseState` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `InstantaneousClose` `V`
- `InstantaneousOpen` `V`
- `IsClosed` `VC`
- `IsOpen` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `Open` `V`
- `PlaceAndClear`
- `PlaceEffectsInWorld`
- `PlayAnimationAndFX`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `StartEffects`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kDefaultAutoClosePeriod` `S`
- `kRemoveBlockerTime` `S`
- `~DynamicDoor` `V`

### `DynamicTeleporter` (Game.dll, 30)

- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `DynamicTeleporter`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HasParentsUniqueId` `V`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `IsStatic` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetActive`
- `SetParentsUniqueId`
- `ShouldServerSpawn` `VC`
- `StopEffects`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~DynamicTeleporter` `V`

### `EditorFilter` (Engine.dll, 21)

- `AddCustomFilter`
- `AddFilterObject`
- `ClearCustomFilter`
- `Destroy` `S`
- `EditorFilter`
- `EditorFilter`
- `Get` `S`
- `GetFilter` `C`
- `GetFilterAll` `C`
- `GetFilterNone` `C`
- `HideShowObject`
- `IsCustomFiltered` `C`
- `IsObjectFiltered` `C`
- `RemoveFilterObject`
- `SetFilter`
- `SetFilterAll`
- `SetFilterNone`
- `Sync`
- ``vftable'`
- `instance` `S`
- `~EditorFilter` `V`

### `Effect` (Engine.dll, 34)

- `AddEmitter`
- `AddToScene`
- `CalculateMemoryUsage` `C`
- `DisableLooping`
- `Effect`
- `Effect`
- `GetFrameScore`
- `GetLight`
- `GetNumEmitters`
- `GetNumParticles`
- `GetProgress` `C`
- `HasActiveParticles`
- `IsComplete`
- `PreLoad`
- `RemoveEmitter`
- `Reset`
- `SetAnchor1`
- `SetAnchor2`
- `SetAnchored`
- `SetBoneList`
- `SetBoundingBox`
- `SetCanBeSoft`
- `SetEditorRegion`
- `SetLocalOrientFix`
- `SetParentEntity`
- `SetPause`
- `SetScale`
- `StartEmitting`
- `StopEmitting`
- `Update`
- ``default constructor closure'`
- ``vftable'`
- `operator=`
- `~Effect` `V`

### `EffectData` (Engine.dll, 28)

- `AddEmitterData`
- `BufferLoad`
- `DropDecalLoad`
- `EffectData`
- `EffectData`
- `GetAllocatedSize` `C`
- `GetBoolean` `C`
- `GetEmitterData`
- `GetEmitterData` `C`
- `GetFloat` `C`
- `GetInteger` `C`
- `GetLoop`
- `GetName`
- `GetNumEmitters` `C`
- `GetString` `C`
- `ImageLoad`
- `Load`
- `RemoveEmitterData`
- `Save`
- `SetBoolean`
- `SetFloat`
- `SetInteger`
- `SetLoop`
- `SetName`
- `SetString`
- `ShaderLoad`
- `operator=`
- `~EffectData`

### `EffectEntity` (Engine.dll, 42)

- `AddToScene` `V`
- `AddToWorld` `V`
- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `DisableCameraShake`
- `DisableLooping`
- `EffectEntity`
- `GetIntersection` `VC`
- `GetRTTIClassInfo` `VC`
- `GetScale` `C`
- `GetStaticClassInfo` `S`
- `HasActiveParticles`
- `InitialUpdate` `V`
- `IsComplete`
- `IsSavedByEditor` `VC`
- `IsSelfDeleting` `C`
- `Load`
- `Load` `V`
- `MakeSelfDeletingChild`
- `PreLoad` `V`
- `PullParentBones`
- `RTTI_new` `S`
- `ReLoad`
- `Reset`
- `ResetDecal`
- `SetAnchor1`
- `SetAnchor2`
- `SetAnchored`
- `SetDynamicLines`
- `SetEffectScale`
- `SetSavedByEditor`
- `SetScale`
- `SetSelfRemove`
- `StartEmitting`
- `StopEmitting`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~EffectEntity` `V`

### `EffectResource` (Engine.dll, 8)

- `Destroy` `V`
- `EffectResource`
- `GetEffectData` `C`
- `GetIsReadyToUse` `C`
- `GetSystemMemoryUsage` `VC`
- `Initialize` `V`
- `InitializeDefault` `V`
- `~EffectResource` `V`

### `Emitter` (Engine.dll, 61)

- `AddToScene`
- `AllocateParticleArray`
- `CalculateMemoryUsage` `C`
- `DisableLooping`
- `DrawDebugLines`
- `EmitAnchoredParticle`
- `EmitBoneParticles`
- `EmitBoneParticles`
- `EmitParticle`
- `EmitParticles`
- `Emitter`
- `Emitter`
- `GetFrameScore`
- `GetIsEmitLocally` `C`
- `GetLight`
- `GetNumParticles`
- `GetNumRenderPasses` `VC`
- `GetProgress` `C`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderForStyle` `C`
- `GetShaderStyle` `VC`
- `GetTexture` `VC`
- `HasActiveParticles` `C`
- `HasEmittedParticles` `C`
- `IncrementXRot`
- `IncrementYRot`
- `IncrementZRot`
- `IsComplete` `C`
- `LogInfo` `VC`
- `PickAnchor` `C`
- `PreLoad`
- `RebuildFaceGroup` `C`
- `ReloadShader`
- `RenderPass` `VC`
- `Reset`
- `SetAnchor1`
- `SetAnchor2`
- `SetAnchored`
- `SetBoundingBox`
- `SetCanBeSoft`
- `SetEditorRegion`
- `SetLocalOrientFix`
- `SetParentEntity`
- `SetPause`
- `SetScale`
- `StartEmitting`
- `StopEmitting`
- `Update`
- `UpdateBoneList`
- `UpdateDropDecal`
- `UpdateForRegionChange`
- `UpdateLight`
- `UpdateParticles`
- `UpdateShader`
- `UpdateTexture`
- ``default constructor closure'`
- ``vftable'`
- `operator=`
- `~Emitter` `V`

### `EmitterData` (Engine.dll, 32)

- `BinaryRead`
- `BinaryWrite`
- `EmitterData`
- `EmitterData`
- `GetAllocatedSize` `C`
- `GetBoolean` `C`
- `GetCurve`
- `GetCurve` `C`
- `GetDebugLines` `C`
- `GetDropDecalName` `C`
- `GetEnabled` `C`
- `GetFloat` `C`
- `GetInteger` `C`
- `GetLoop` `C`
- `GetShaderName` `C`
- `GetString` `C`
- `GetTextureName` `C`
- `InternalBinaryRead`
- `OldBinaryRead`
- `SetBoolean`
- `SetDebugLines`
- `SetDropDecalName`
- `SetEnabled`
- `SetFloat`
- `SetInteger`
- `SetLoop`
- `SetShaderName`
- `SetString`
- `SetTextureName`
- `UpdateShader`
- `operator=`
- `~EmitterData`

### `EnchanterDismantleConfigCmd` (Game.dll, 7)

- `EnchanterDismantleConfigCmd`
- `EnchanterDismantleConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~EnchanterDismantleConfigCmd` `V`

### `EnchanterDismantleConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EnchanterDismantleConfigCmdPacket`
- `EnchanterDismantleConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EnchanterDismantleConfigCmdPacket` `V`

### `EnchanterRecoveryConfigCmd` (Game.dll, 7)

- `EnchanterRecoveryConfigCmd`
- `EnchanterRecoveryConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~EnchanterRecoveryConfigCmd` `V`

### `EnchanterRecoveryConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EnchanterRecoveryConfigCmdPacket`
- `EnchanterRecoveryConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EnchanterRecoveryConfigCmdPacket` `V`

### `EnchanterTinkerConfigCmd` (Game.dll, 7)

- `EnchanterTinkerConfigCmd`
- `EnchanterTinkerConfigCmd`
- `Execute` `V`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~EnchanterTinkerConfigCmd` `V`

### `EnchanterTinkerConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EnchanterTinkerConfigCmdPacket`
- `EnchanterTinkerConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EnchanterTinkerConfigCmdPacket` `V`

### `EndlessBuffShrine` (Game.dll, 27)

- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `CleanseShrine` `V`
- `EndlessBuffShrine`
- `GetBuffSkillName` `C`
- `GetGameDescription` `VC`
- `GetRTTIClassInfo` `VC`
- `GetShrineType` `C`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~EndlessBuffShrine` `V`

### `EndlessDungeon_Generator` (Game.dll, 63)

- `AddBossTrackerId`
- `Clear`
- `DisableProxies`
- `EndlessDungeon_Generator`
- `EndlessDungeon_Generator`
- `GenerateDungeon`
- `GetActiveFloor` `C`
- `GetActiveFloorEntrance` `C`
- `GetAllowedItems` `C`
- `GetBlacklistedBossPools` `C`
- `GetBlacklistedHeroes` `C`
- `GetBlacklistedShrinePools` `C`
- `GetBonusTime` `C`
- `GetDbrName` `C`
- `GetDifficultyAdjustment` `C`
- `GetDistressCallGroup` `C`
- `GetDistressCallRange` `C`
- `GetDistressResponseGroup` `C`
- `GetDropsDisabled` `C`
- `GetDungeonId` `C`
- `GetDungeonProgress` `C`
- `GetDyingPenalty` `C`
- `GetEntranceId` `C`
- `GetFactionOverride` `C`
- `GetFloorNumber` `C`
- `GetFloorProgress` `C`
- `GetHealthGainOnKillPct` `C`
- `GetMaxFloorProgress` `C`
- `GetMiscDropItem`
- `GetProgressItem` `C`
- `GetRevealMonsters` `C`
- `GetRoamBehavior` `C`
- `GetViewDistance` `C`
- `IncrementBonusTimeKillCounter`
- `IncrementProgress`
- `IsBonusComplete` `C`
- `IsBossDead` `C`
- `IsBossRegion` `C`
- `IsReallyDead` `S`
- `IsRegionInDungeon` `C`
- `IsTreasureRoom` `C`
- `KillMonstersInRegion` `C`
- `LoadFromDatabase`
- `LoadProxies`
- `OpenExit`
- `PickExits`
- `PickFloors`
- `PickProxies`
- `ResolveEquationVariable` `VC`
- `RestartDungeon`
- `SetActiveFloor`
- `SetBossDead`
- `SetDungeonId`
- `SetDungeonProgress`
- `SetEntranceId`
- `SetFloorProgress`
- `Update`
- ``vftable'`
- `kBlacklistBossPoolSize` `S`
- `kMaxFloorQueueSize` `S`
- `kbossDeathCheck` `S`
- `operator=`
- `~EndlessDungeon_Generator`

### `EngageAltarRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageAltarRequestPacket`
- `EngageAltarRequestPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageAltarRequestPacket` `V`

### `EngageAltarResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageAltarResponsePacket`
- `EngageAltarResponsePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageAltarResponsePacket` `V`

### `EngageContainerRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageContainerRequestPacket`
- `EngageContainerRequestPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageContainerRequestPacket` `V`

### `EngageContainerResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageContainerResponsePacket`
- `EngageContainerResponsePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageContainerResponsePacket` `V`

### `EngageEndlessShrineRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageEndlessShrineRequestPacket`
- `EngageEndlessShrineRequestPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageEndlessShrineRequestPacket` `V`

### `EngageEndlessShrineResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageEndlessShrineResponsePacket`
- `EngageEndlessShrineResponsePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageEndlessShrineResponsePacket` `V`

### `EngageNpcAction` (Game.dll, 7)

- `EngageNpcAction`
- `EngageNpcAction`
- `Execute` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- ``vftable'`
- `~EngageNpcAction` `V`

### `EngageNpcPacket` (Game.dll, 8)

- `Deserialize` `V`
- `EngageNpcPacket`
- `EngageNpcPacket`
- `GetPacketDescription` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~EngageNpcPacket` `V`

### `EngageNpcRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageNpcRequestPacket`
- `EngageNpcRequestPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageNpcRequestPacket` `V`

### `EngageNpcResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageNpcResponsePacket`
- `EngageNpcResponsePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageNpcResponsePacket` `V`

### `EngageShrineRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageShrineRequestPacket`
- `EngageShrineRequestPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageShrineRequestPacket` `V`

### `EngageShrineResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EngageShrineResponsePacket`
- `EngageShrineResponsePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EngageShrineResponsePacket` `V`

### `Engine` (Engine.dll, 238)

- `AddLog`
- `AddStatisticGraph`
- `AddStatisticText`
- `AddWidget`
- `AllowAlphaToCoverage`
- `AllowLightrigSpec` `C`
- `Break`
- `CharacterCommandLogEnable`
- `CharacterDataLogEnable`
- `CleanseDatabase`
- `ClearCinematicText`
- `CloseLogFiles`
- `CloudSaveClear`
- `Crash`
- `CreateLogCategory`
- `CreateNetPacket`
- `DecrementPathingCounter`
- `DisableCharacterProcessing`
- `DisableLog`
- `DisplayD3D9Stats`
- `DoCharacterProcessing` `C`
- `DumpCharacterStatsToFile`
- `DumpHostTable`
- `DumpNetworkStats`
- `DumpProfiler`
- `EnableCharacterProcessing`
- `EnableD3D9Stats`
- `EnableLog`
- `EnablePerfStats`
- `EnableTimeSmoothing`
- `Engine`
- `EvictOldResources`
- `Expansion1FilesExist` `C`
- `Expansion2FilesExist` `C`
- `Expansion3FilesExist` `C`
- `GetAlternateKeymapPath` `C`
- `GetAreaNameTag` `C`
- `GetAtomicCommit` `C`
- `GetBaseDataPathFromOptions`
- `GetBaseUserPath` `C`
- `GetCDKey` `C`
- `GetChunkCallbackMap`
- `GetCinematicTextEvents` `C`
- `GetControllerDebugging` `C`
- `GetConversationResourceManager`
- `GetDatabaseArchive`
- `GetDatabaseArchiveChecksum`
- `GetDefaultForwardShadowBias` `C`
- `GetDefaultSettingsPath` `C`
- `GetDeferredDirectionalShadowBias` `C`
- `GetDeferredPointShadowBias` `C`
- `GetDeferredRenderer` `C`
- `GetDepthOfFieldRange` `C`
- `GetDumpPath` `C`
- `GetEffectResourceManager`
- `GetEntitiesInPriorFrameFrustum`
- `GetEntityRenderFilter`
- `GetFileSystem`
- `GetFontDebugging` `C`
- `GetForcedUpdateEntities`
- `GetFrameCount` `C`
- `GetFxQualityModifier` `C`
- `GetGameInfo`
- `GetGraphicsEngine` `C`
- `GetIdle` `C`
- `GetImageResourceManager`
- `GetInputDevice`
- `GetIsSingleProcessorModeEnabled` `C`
- `GetKeymapPath` `C`
- `GetLastUserInputTime` `C`
- `GetLevelLoadInflation` `S`
- `GetLevelUpdateInflation` `S`
- `GetLoadSphereInflation` `S`
- `GetLoadedExpansionId` `C`
- `GetLogCategoryName` `C`
- `GetLogPath` `C`
- `GetMPVersion`
- `GetNetworkConnectionManager`
- `GetNetworkController`
- `GetNetworkEntityList`
- `GetNetworkShim`
- `GetNormalSettingsPath` `C`
- `GetOptions`
- `GetPathMeshErrorBuffer`
- `GetPathingCounter`
- `GetPerfTracker`
- `GetPhysicsEngine`
- `GetRegionOfNote` `C`
- `GetRenderDataLoadInflation` `S`
- `GetResourceLoader`
- `GetSSAOBlur` `C`
- `GetSSAOScale` `C`
- `GetSSAOStrength` `C`
- `GetSaveManager`
- `GetSavePath` `C`
- `GetTrailDebugging` `C`
- `GetUpdateTime` `C`
- `GetUseHighPrecisionDepthBuffer` `C`
- `GetUtilityFontStyle` `C`
- `GetVersion` `S`
- `HasLoadedCustomDatabase` `C`
- `HasLoyalistPack1DLC` `C`
- `HasLoyalistPack2DLC` `C`
- `HasSurvivalDLC` `C`
- `ImplantStatisticRunGameInfo`
- `IncrementPathingCounter`
- `Initialize` `V`
- `InitializeAssertHandler`
- `InitializeMod`
- `InstallMapChunkCallback`
- `InternalLog` `C`
- `IsCDKeyValid`
- `IsCharacterCommandLogEnabled`
- `IsCharacterDataLogEnabled`
- `IsDedicatedServer` `C`
- `IsEditorMode`
- `IsExpansion1Enabled` `C`
- `IsExpansion1Loaded` `C`
- `IsExpansion2Enabled` `C`
- `IsExpansion2Loaded` `C`
- `IsExpansion3Enabled` `C`
- `IsExpansion3Loaded` `C`
- `IsExpansionSet`
- `IsInDeviceReset`
- `IsInTeleportMode` `C`
- `IsLoadingDbFromArchive`
- `IsMapRendering`
- `IsNetworkClient`
- `IsNetworkEnabled`
- `IsNetworkServer`
- `IsPSEditorMode`
- `IsProfilerShown` `C`
- `IsRenderDebuggerShown` `C`
- `IsRunningCharacterImport` `C`
- `IsRunningGameSimulation` `C`
- `IsUserIdle` `C`
- `LoadAdditionalDatabases`
- `LoadCustomMapDatabase`
- `LoadDatabase`
- `LoadMainDatabase`
- `LoadSurvival1Database`
- `LoadSurvival2Database`
- `LoadSurvival3Database`
- `Log` `VC`
- `Log` `VC`
- `LogCloudSaveFiles`
- `LogNames`
- `LogNetworkEntities`
- `LogPublicIP`
- `LogRegionStates`
- `LogSeparator` `VC`
- `LoyalistPack1FilesExist` `C`
- `LoyalistPack2FilesExist` `C`
- `NetworkFrustumStats`
- `PostDeviceReset`
- `PreDeviceReset`
- `PresentSurface`
- `PrintCoreInfo`
- `ProcessUserInput`
- `RegisterForForcedUpdates`
- `RegisterResetObject`
- `RemoveLog`
- `RemoveLogCategory`
- `RemoveModDirectory`
- `RemoveWidget`
- `Render`
- `ResetGameTimer`
- `SaveScreenShot` `C`
- `SetAlternateKeymapPath`
- `SetAreaNameTag`
- `SetAsNetworkClient`
- `SetAsNetworkServer`
- `SetAsNetworkStub`
- `SetAssertHandlerLogPath`
- `SetBaseUserPath`
- `SetCDKey`
- `SetControllerDebugging`
- `SetDefaultSettingsPath`
- `SetDefaultShadowBias`
- `SetDeferredDirectionalShadowBias`
- `SetDeferredPointShadowBias`
- `SetDeferredRenderer`
- `SetDeviceLost`
- `SetDumpPath`
- `SetEditorMode`
- `SetEntityRenderFilter`
- `SetExpansionFlag`
- `SetFileSystem`
- `SetFontDebugging`
- `SetFxQualityModifier`
- `SetIdle`
- `SetKeymapPath`
- `SetLightrigSpec`
- `SetLogPath`
- `SetLuaGlueLogging`
- `SetMapRendering`
- `SetMultithreadedRendering`
- `SetNetPacketCreator`
- `SetNetworkController`
- `SetNetworkLogging`
- `SetNetworkStats`
- `SetNormalSettingsPath`
- `SetPSEditorMode`
- `SetPathMeshErrorBuffer`
- `SetRegionOfNote`
- `SetRunningCharacterImport`
- `SetRunningGameSimulation`
- `SetSSAOParams`
- `SetSavePath`
- `SetTeleportMode`
- `SetTrailDebugging`
- `ShowCinematicText`
- `ShowMemViewer`
- `ShowNetworkDisplay`
- `ShowProfiler`
- `ShowRenderDebugger`
- `ShowThreadMonitor`
- `Shutdown`
- `ShutdownNetwork`
- `SurvivalFilesExist` `C`
- `UnloadAllRegions`
- `UnloadUnreferencedResources`
- `UnloadWorld`
- `UnregisterForForcedUpdates`
- `UnregisterResetObject`
- `Update`
- `UpdateClientEntities`
- `UpdateForcedEntities`
- `UpdateForcedEntitiesInPlayerLoadSphere`
- `UpdateFrustum`
- `UpdatePerfTracker`
- `UserFileExists` `C`
- `WriteMemLog`
- `WriteMemTable`
- `WritePerformanceStatsFile`
- `WriteResourceLog`
- ``vftable'`
- `~Engine` `V`

### `Entity` (Engine.dll, 165)

- `Activate`
- `AddAttachedEntitiesToScene` `V`
- `AddForcedEntitiesToScene` `V`
- `AddToScene` `V`
- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `Attach` `V`
- `Attach` `V`
- `AttachIllumination`
- `AttachPunctuation`
- `BillboardPunctuation` `V`
- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `CanBePlacedInEditor` `VC`
- `CanTriggerBoundingVolumes` `C`
- `CheckForcedUpdateLoadSphere` `VC`
- `CheckLOS` `VC`
- `CollisionCallback` `V`
- `CreateAttachedEntity`
- `CreatePathObstacles` `V`
- `CreateSpawnNetPacket` `V`
- `CrowdAgentCreated` `V`
- `CrowdAgentDestroyed` `V`
- `CrowdAgentError` `V`
- `CrowdAgentMoved` `V`
- `CrowdAgentReachedGoal` `V`
- `CrowdAgentStopped` `V`
- `CrowdAgentUpdate` `V`
- `DeleteOnEnteringUnloadedLevel` `VC`
- `Detach`
- `DetachIllumination`
- `DetachPunctuation`
- `DynamicPathingOccluder` `VC`
- `EnsureUniqueID`
- `Entity`
- `ExecuteEventHook` `V`
- `FastUpdate` `V`
- `ForcedUpdate` `V`
- `GetAttachedCoords` `VC`
- `GetAttachedCoordsInRegion` `VC`
- `GetAttachedEntity`
- `GetBoneCoordsInRegion` `VC`
- `GetCenterOfMass` `VC`
- `GetCollisionBox` `VC`
- `GetCollisionShape` `VC`
- `GetCollisionType` `VC`
- `GetCoords` `C`
- `GetCoordsUnparented` `C`
- `GetExtents` `VC`
- `GetGravityEnabled` `C`
- `GetHitBox` `VC`
- `GetHitBox` `VC`
- `GetIntersection` `VC`
- `GetIntersection` `VC`
- `GetIsPartOfLevel` `VC`
- `GetLastFrameUpdated` `C`
- `GetMotion` `C`
- `GetNumAttachedEntities`
- `GetNumHitBoxes` `VC`
- `GetNumUpdates` `C`
- `GetOwner` `C`
- `GetParent` `C`
- `GetPhysicsFriction` `VC`
- `GetPhysicsMass` `VC`
- `GetPhysicsMesh` `VC`
- `GetPhysicsRestitution` `VC`
- `GetPhysicsSimulation` `C`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetRegion` `C`
- `GetRegionBoundingBox` `C`
- `GetRenderFar` `C`
- `GetShouldRenderAcrossPortals` `VC`
- `GetStaticClassInfo` `S`
- `GetStoredOrientation` `V`
- `GetUICloseDistance` `VC`
- `GetUniqueID`
- `GetVisibility` `VC`
- `HasDoneInitialUpdate` `C`
- `HasParentsUniqueID` `VC`
- `HasUniqueID` `C`
- `InRenderPreLoadFrustum` `C`
- `IncludeInMinimap` `VC`
- `InitialUpdate` `V`
- `IsActivated`
- `IsAttachedFromMesh` `C`
- `IsCopy` `VC`
- `IsInSpace`
- `IsInWorld` `VC`
- `IsOwnedByClient`
- `IsQuestRelated` `C`
- `IsRestored` `C`
- `IsSavedByEditor` `VC`
- `IsStatic` `VC`
- `IsStationaryAttachment` `C`
- `IsVisible` `C`
- `Load` `V`
- `LoadWithoutScripts` `V`
- `LogInfo` `VC`
- `OccludesPathing` `VC`
- `OnAddToLevel` `V`
- `OnAddToWorld` `V`
- `OnDestroy` `V`
- `OnMoveInLevel` `V`
- `OnMoved` `V`
- `OnPathFailed` `V`
- `OnReachedMovementGoal` `V`
- `OnRemoveFromLevel` `V`
- `OnRemoveFromWorld` `V`
- `OnUnload` `V`
- `Owns`
- `PhysicsCollision` `V`
- `PhysicsIsSimulated` `C`
- `PhysicsPost` `V`
- `PhysicsResponse` `V`
- `PhysicsSetup` `V`
- `PhysicsTest` `V`
- `PhysicsUpdate` `V`
- `PlaySyncedAnimation` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Read` `V`
- `ReadReplicationData` `V`
- `RegisterAnimationCallback` `V`
- `RemoveAttachedEntity`
- `RemovePathObstacles` `V`
- `RemoveUniqueID`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetAttachOffset`
- `SetCoords`
- `SetDynamicObstacle` `V`
- `SetGravityEnabled`
- `SetInRenderPreLoadFrustum`
- `SetMotion`
- `SetNumUpdates`
- `SetOwner`
- `SetPhysicsSimulation` `V`
- `SetRegionBoundingBox`
- `SetRestored`
- `SetSaveState` `V`
- `SetShouldServerSpawn`
- `SetStationaryAttachment`
- `SetTransparency` `V`
- `SetTransparent` `V`
- `SetUniqueID`
- `SetVisibility` `V`
- `SetVisibility` `V`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `Update`
- `UpdateAttachedEntities`
- `UpdateBoundingBox` `V`
- `UpdatePunctuation` `V`
- `UpdateSelf` `V`
- `UseExistingObjectForRestore` `VC`
- `UseStoredOrientation` `VC`
- `Write` `VC`
- `WriteReplicationData` `V`
- `WriteSimulationInformation` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Entity` `V`

### `EnumConverter` (Game.dll, 8)

- `GetEnumAsString` `S`
- `GetEnumAsString` `S`
- `GetEnumAsString` `S`
- `GetStringAsEnum<enum GAME::ActorMountType>` `S`
- `GetStringAsEnum<enum GAME::CharacterPathGenerationStyle>` `S`
- `GetStringAsEnum<enum GAME::PlayerCharacterClassType>` `S`
- `operator=`
- `operator=`

### `EquipManager` (Game.dll, 45)

- `AddPropAttachment`
- `AttachItemAction`
- `AttachParent`
- `CalculateAllocatedMemory` `C`
- `CreateDefaultPiece` `C`
- `DetachItemAction`
- `EquipManager`
- `ForceRightHandAsNeeded`
- `GetAllArmor` `C`
- `GetAllNonWeapons` `C`
- `GetArmorId` `C`
- `GetAttachCoords` `C`
- `GetAttachedItems` `C`
- `GetBodyArmor` `C`
- `GetCompatible`
- `GetDefaultEquip` `C`
- `GetEquipCtrlLocation` `S`
- `GetEquipLocation` `S`
- `GetHandState` `C`
- `GetItemAlternate` `C`
- `GetItemCount` `C`
- `GetItemCtrlLocation` `C`
- `GetItemId` `C`
- `GetItemLocation` `C`
- `GetLeftHandShield` `C`
- `GetLeftHandWeapon` `C`
- `GetRightHandWeapon` `C`
- `GetWeaponIdLeft` `C`
- `GetWeaponIdRight` `C`
- `HasItem` `C`
- `HasItem` `C`
- `HasItemEquipped` `C`
- `HideAttachments`
- `Load`
- `LocationToString` `C`
- `PreLoad`
- `RemoveItem`
- `RemovePropAttachment`
- `SetWeaponEnchantment`
- `ShowAttachments`
- `StringToLocation` `C`
- `UpdateAnimations`
- `UpdateEquipVisibility`
- `UpdateWeaponFxVisibility`
- `~EquipManager`

### `EquipManagerContainer` (Game.dll, 4)

- `EquipManagerContainer`
- `operator<` `C`
- `operator=`
- `~EquipManagerContainer`

### `EquipmentCtrl` (Game.dll, 43)

- `AreRequirementsMet` `C`
- `AttachItem`
- `AttributesHaveChanged`
- `AutoInsertItem`
- `CanItemBePlaced` `C`
- `CanItemBePlaced_HandLeft` `C`
- `CanItemBePlaced_HandRight` `C`
- `CanItemBePlaced_Slot` `C`
- `DetachItem`
- `EquipmentCtrl`
- `EquipmentCtrl`
- `GetEquipmentLocationTag` `S`
- `GetEquippedLocation` `C`
- `GetIsAlternate` `C`
- `GetItemId` `C`
- `GetItemId_HandLeft` `C`
- `GetItemId_HandRight` `C`
- `GetItemId_Slot` `C`
- `GetItemSaveInfo` `C`
- `GetItem_HandLeft` `C`
- `GetItem_HandRight` `C`
- `IsEquipSlotEmpty` `C`
- `IsItemAttached` `C`
- `PlaceItem`
- `PlaceItem_HandLeft`
- `PlaceItem_HandRight`
- `PlaceItem_Slot`
- `ReadProperties`
- `RemoveItem`
- `RestoreItem`
- `SetAllowDualWeapons`
- `SetAlternateEquipment`
- `SetController`
- `SetIgnoreRequirements`
- `SetParent`
- `Sift`
- `SmartAutoInsert`
- `StreamProperties`
- `ValidateItemType` `C`
- `WriteProperties` `C`
- ``vftable'`
- `operator=`
- `~EquipmentCtrl` `V`

### `EvadeAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `EvadeAction`
- `EvadeAction`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- ``vftable'`
- `~EvadeAction` `V`

### `EvadeActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `EvadeActionPacket`
- `EvadeActionPacket`
- `GetPacketDescription` `V`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~EvadeActionPacket` `V`

### `Event` (Engine.dll, 8)

- `Event`
- `GetSignaled` `C`
- `Reset`
- `SetSignaled`
- `Wait`
- `WaitForAny` `S`
- ``default constructor closure'`
- `~Event` `V`

### `EventHookCommandPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `EventHookCommandPacket`
- `EventHookCommandPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~EventHookCommandPacket` `V`

### `EventManager` (Engine.dll, 7)

- `EnableDebugging`
- `EventManager`
- `Register`
- `Send`
- `UnRegister`
- ``vftable'`
- `~EventManager` `V`

### `ExperienceNotificationPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `ExperienceNotificationPacket`
- `ExperienceNotificationPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~ExperienceNotificationPacket` `V`

### `FFDGizmo` (Engine.dll, 16)

- `FFDGizmo`
- `FFDGizmo`
- `GetFFDBasePoint`
- `GetFFDControlPoint`
- `OffsetFFDControlPoint`
- `RenderDebugFFD` `C`
- `RenderFFDBox` `C`
- `SetFFDAmplitude`
- `SetFFDBasePoint`
- `SetFFDControlPoint`
- `SetFFDDefault`
- `SetFFDExponent`
- `SetFFDFrequency`
- `SetShaderParameters` `C`
- `operator=`
- `~FFDGizmo`

### `FOWManager` (Engine.dll, 12)

- `FOWManager`
- `FOWManager`
- `IsDirty` `C`
- `Read`
- `Reset`
- `Retrieve` `C`
- `Store`
- `Write` `C`
- `kMagic` `S`
- `kVersion` `S`
- `operator=`
- `~FOWManager`

### `FactionManager` (Game.dll, 20)

- `DefaultMonsterFaction` `S`
- `DefaultPlayerFaction` `S`
- `FactionManager`
- `GetDebug` `C`
- `GetPvpIds` `S`
- `GiveFactionValue`
- `InvisibleFaction` `S`
- `IsFoe`
- `IsFoe`
- `IsFoe`
- `IsFoe`
- `IsFriend`
- `IsFriend`
- `IsFriend`
- `IsFriend`
- `SetAllInvisible`
- `SetDebug`
- `SetFactionValue`
- `UnlockFaction`
- `~FactionManager`

### `FactionPack` (Game.dll, 51)

- `AddToPacket` `C`
- `Adjust`
- `AdjustValue`
- `DeserializeFromString`
- `FactionClamp` `C`
- `FactionPack`
- `FactionPack`
- `FactionPack`
- `GetBountyEnabled` `C`
- `GetDisplay` `C`
- `GetFaction` `C`
- `GetFactionFromString` `S`
- `GetFactionIcon` `C`
- `GetFactionInfoTag` `S`
- `GetFactionLargeIcon` `C`
- `GetFactionTag` `S`
- `GetKillAdjustment` `C`
- `GetNegativeBoosted`
- `GetParent`
- `GetPositiveBoosted`
- `GetQuestEnabled` `C`
- `GetTierReward` `C`
- `GetValue` `C`
- `IncrementNemesisSpawnCounter`
- `IsFoe` `C`
- `IsFriend` `C`
- `IsGood` `C`
- `IsModified` `C`
- `IsNegativeBoosted` `C`
- `IsPositiveBoosted` `C`
- `IsUnlocked` `C`
- `Load`
- `ReadProperties`
- `ReadReplicationData`
- `RemoveFromPacket`
- `ResetNemesis`
- `SerializeToString`
- `SetFaction`
- `SetNegativeBoosted`
- `SetParent`
- `SetPositiveBoosted`
- `SetValue`
- `StreamProperties`
- `UnlockFaction`
- `WriteProperties` `C`
- `WriteReplicationData`
- ``vftable'`
- `alone` `S`
- `legacyFactionNum` `S`
- `operator=`
- `~FactionPack` `V`

### `FactionToClientPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FactionToClientPacket`
- `FactionToClientPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FactionToClientPacket` `V`

### `FadeActorDeathHandler` (Game.dll, 9)

- `Execute` `V`
- `FadeActorDeathHandler`
- `FadeActorDeathHandler`
- `IsOverideAllowed` `VC`
- `PostProcess` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~FadeActorDeathHandler` `V`

### `File` (Engine.dll, 5)

- `File`
- `File`
- ``vftable'`
- `operator=`
- `~File` `V`

### `FileBrowserWindow` (Widget.dll, 10)

- `Create`
- `GetSelectedItems` `C`
- `OnChildEvent` `V`
- `OnCreate` `V`
- `OnSizeChange` `V`
- `SetFileSystem`
- `SetMultipleSelection`
- `SetSource`
- `ShowFiles`
- `ShowRecords`

### `FileDirectory` (Engine.dll, 9)

- `FileDirectory`
- `GetLength` `VC`
- `GetSource` `VC`
- `Lock` `V`
- `Lock` `V`
- `Open`
- `Unlock` `V`
- `Write` `V`
- `~FileDirectory` `V`

### `FileSource` (Engine.dll, 5)

- `FileSource`
- `FileSource`
- ``vftable'`
- `operator=`
- `~FileSource` `V`

### `FileSourceArchive` (Engine.dll, 19)

- `Attach`
- `CloseFile` `V`
- `CloseFileStream` `V`
- `Detach`
- `FileSourceArchive`
- `FindFilePath` `VC`
- `GetArchive`
- `GetArchiveRelativePath` `C`
- `GetFileTime` `VC`
- `GetFullPath` `VC`
- `GetMatchingFiles` `V`
- `GetName` `VC`
- `IsFileWriteable` `VC`
- `OpenFile` `V`
- `OpenFileStream` `V`
- `OpenFullFilePath` `V`
- ``default constructor closure'`
- ``vftable'`
- `~FileSourceArchive` `V`

### `FileSourceDirectory` (Engine.dll, 13)

- `Attach`
- `CloseFile` `V`
- `CloseFileStream` `V`
- `FindFilePath` `VC`
- `GetFileTime` `VC`
- `GetFullPath` `VC`
- `GetMatchingFiles` `V`
- `GetName` `VC`
- `IsFileWriteable` `VC`
- `OpenFile` `V`
- `OpenFileStream` `V`
- `OpenFullFilePath` `V`
- `~FileSourceDirectory` `V`

### `FileSystem` (Engine.dll, 25)

- `AddSource`
- `AddSourceArchive`
- `CloseFile` `V`
- `CloseFileStream` `V`
- `CompareFileTimes` `C`
- `CompareFileTimes` `C`
- `DebugLog`
- `DoesFileExist`
- `FileSystem`
- `FileSystem`
- `FindFullFilePath`
- `GetFileList`
- `GetFileSize`
- `GetFileTime` `VC`
- `GetMatchingFiles`
- `IsFileWriteable` `VC`
- `OpenFile` `V`
- `OpenFileStream` `V`
- `OpenFileWithCheck` `V`
- `OpenFullFilePath` `V`
- `RemoveSourcesFromPartition`
- `ValidateFileName` `S`
- ``vftable'`
- `operator=`
- `~FileSystem` `V`

### `FilterCharacterAction` (Game.dll, 7)

- `FilterCharacterAction`
- `FilterCharacterAction`
- `FilterCharacterAction`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `FilterFunction<GAME::CharacterAction>` (Game.dll, 5)

- `FilterFunction<GAME::CharacterAction>`
- `FilterFunction<GAME::CharacterAction>`
- ``vftable'`
- `operator=`
- `operator=`

### `FilterFunction<GAME::Entity>` (Engine.dll, 5)

- `FilterFunction<GAME::Entity>`
- `FilterFunction<GAME::Entity>`
- ``vftable'`
- `operator=`
- `operator=`

### `FilterFunction<GAME::Entity>` (Game.dll, 5)

- `FilterFunction<GAME::Entity>`
- `FilterFunction<GAME::Entity>`
- ``vftable'`
- `operator=`
- `operator=`

### `FilterGetAllPlayers` (Game.dll, 7)

- `FilterGetAllPlayers`
- `FilterGetAllPlayers`
- `FilterGetAllPlayers`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `FilterGetPlayerSpawnPoint` (Game.dll, 7)

- `FilterGetPlayerSpawnPoint`
- `FilterGetPlayerSpawnPoint`
- `FilterGetPlayerSpawnPoint`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `FilterMountAction` (Game.dll, 7)

- `FilterMountAction`
- `FilterMountAction`
- `FilterMountAction`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `FilterMountedCharacterAction` (Game.dll, 7)

- `FilterMountedCharacterAction`
- `FilterMountedCharacterAction`
- `FilterMountedCharacterAction`
- ``vftable'`
- `operator()` `VC`
- `operator=`
- `operator=`

### `FireLight` (Engine.dll, 13)

- `FireLight`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnAddToLevel` `V`
- `OnMoveInLevel` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FireLight` `V`

### `FixedActor` (Game.dll, 22)

- `FixedActor`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUsableRange` `C`
- `InitialUpdate` `V`
- `IsEntityAtFront` `C`
- `IsOfInterest` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `ShouldSaveState` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kDefaultUsableRange` `S`
- `~FixedActor` `V`

### `FixedDoor` (Game.dll, 45)

- `AnimationCallback` `V`
- `ApplyReplicationData` `V`
- `CanInteract` `VC`
- `CanInteractWithEntity` `VC`
- `CheckLOS` `VC`
- `Close` `V`
- `CreatePathObstacles` `V`
- `FixedDoor`
- `GetGameDescription` `VC`
- `GetOpenCloseState` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `InstantaneousClose` `V`
- `InstantaneousOpen` `V`
- `IsClosed` `VC`
- `IsLocked` `C`
- `IsOfInterest` `VC`
- `IsOpen` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `Open` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RemovePathObstacles` `V`
- `RequestToUse` `V`
- `ResolveEnum_Direction` `S`
- `ResolveEnum_Interaction` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetAutoClose`
- `SetDirection`
- `SetLocked`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kDefaultAutoClosePeriod` `S`
- `kRemoveBlockerTime` `S`
- `~FixedDoor` `V`

### `FixedItem` (Game.dll, 56)

- `AnimationCallback` `V`
- `ApplyReplicationData` `V`
- `CreatePathObstacle` `V`
- `DebugRender` `VC`
- `DeleteOnEnteringUnloadedLevel` `VC`
- `FixedItem`
- `GetDropPerPlayerItem` `C`
- `GetGameDescription` `VC`
- `GetIsPartOfLevel` `VC`
- `GetLootDropCoords` `C`
- `GetLootDropRadius` `C`
- `GetMoveToPoint` `VC`
- `GetProxyLevel` `C`
- `GetRTTIClassInfo` `VC`
- `GetSkill` `VC`
- `GetSkillDelay` `VC`
- `GetStaticClassInfo` `S`
- `GiveBonusToTargets`
- `InitialUpdate` `V`
- `IsAlternateLock` `VC`
- `IsAutoCloseEnabled` `VC`
- `IsCharacterInFront` `VC`
- `IsLocked` `VC`
- `IsStatic` `VC`
- `Load` `V`
- `NotifyAllies` `C`
- `OccludesPathing` `VC`
- `OnAddToLevel` `V`
- `OnDestroy` `V`
- `OnMoveInLevel` `V`
- `OnRemoveFromLevel` `V`
- `PlayLockedSound`
- `PlaySkillWarningFx`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RemovePathObstacle` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetCurrentState`
- `SetDropPerPartyMemberItem`
- `SetFixedItemState` `V`
- `SetLock` `V`
- `SetProxyLevel`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `UseOnTarget` `V`
- `UseSkillOnTargets` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItem` `V`

### `FixedItemBlastContainer` (Game.dll, 25)

- `AddSocialTarget` `V`
- `Converse` `V`
- `DeleteSocialTarget` `V`
- `FixedItemBlastContainer`
- `GetRTTIClassInfo` `VC`
- `GetSocialTarget` `V`
- `GetStaticClassInfo` `S`
- `HasConversation` `C`
- `IsAvailableForConversations` `C`
- `IsChatting` `V`
- `IsChattingWithPlayer`
- `Load` `V`
- `OnConversationEnd`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToOpen`
- `RequestToUse` `V`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `UpdateSocialTargetList` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemBlastContainer` `V`

### `FixedItemContainer` (Game.dll, 21)

- `AppendDetailMapData` `V`
- `CheckRadiusClear`
- `ExecuteEventHook` `V`
- `FixedItemContainer`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `Open`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `SetFixedItemState` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemContainer` `V`

### `FixedItemContainerController` (Game.dll, 11)

- `AnimationCallback` `V`
- `FixedItemContainerController`
- `FixedItemContainerController`
- `InitialUpdate` `V`
- `LoadFromDatabase` `V`
- `TouchedByActor` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `operator=`
- `~FixedItemContainerController` `V`

### `FixedItemController` (Game.dll, 18)

- `FixedItemController`
- `FixedItemController`
- `GetLootDropCoords` `VC`
- `GetLootDropGroup` `VC`
- `GetLootDropRadius` `VC`
- `GetProxyLevel` `C`
- `HasBeenTouchedByActor` `V`
- `LoadDropLoot`
- `LoadFromDatabase` `V`
- `ResolveEquationVariable` `VC`
- `RunSkill`
- `SetFixedItemState` `V`
- `SetParent` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `operator=`
- `~FixedItemController` `V`

### `FixedItemDoor` (Game.dll, 16)

- `AnimationCallback` `V`
- `Close` `V`
- `FixedItemDoor`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InstantaneousClose` `V`
- `InstantaneousOpen` `V`
- `Load` `V`
- `Open` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemDoor` `V`

### `FixedItemDoorBase` (Game.dll, 28)

- `CheckLOS` `VC`
- `Close` `V`
- `FixedItemDoorBase`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `InstantaneousClose` `V`
- `InstantaneousOpen` `V`
- `IsAlternateLock` `VC`
- `IsAutoCloseEnabled` `VC`
- `IsOfInterest` `VC`
- `Load` `V`
- `Open` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetFixedItemState` `V`
- `ShouldSaveState` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kRemoveBlockerTime` `S`
- `~FixedItemDoorBase` `V`

### `FixedItemDoorController` (Game.dll, 11)

- `AnimationCallback` `V`
- `FixedItemDoorController`
- `FixedItemDoorController`
- `InitialUpdate` `V`
- `LoadFromDatabase` `V`
- `TouchedByActor` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `operator=`
- `~FixedItemDoorController` `V`

### `FixedItemDoorSwapping` (Game.dll, 15)

- `Close` `V`
- `FixedItemDoorSwapping`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InstantaneousClose` `V`
- `InstantaneousOpen` `V`
- `Load` `V`
- `Open` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemDoorSwapping` `V`

### `FixedItemDungeonTeleport` (Game.dll, 17)

- `AppendDetailMapData` `V`
- `FixedItemDungeonTeleport`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetDungeonId`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemDungeonTeleport` `V`

### `FixedItemGravestone` (Game.dll, 26)

- `ApplyReplicationData` `V`
- `FixedItemGravestone`
- `GetGameDescription` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `PlayAnim`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetOwner`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemGravestone` `V`

### `FixedItemQuestObject` (Game.dll, 18)

- `AnimationCallback` `V`
- `FixedItemQuestObject`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `SetLock` `V`
- `SetOperation`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemQuestObject` `V`

### `FixedItemShrine` (Game.dll, 29)

- `AnimationCallback` `V`
- `AppendDetailMapData` `V`
- `FixedItemShrine`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GoActive`
- `GoActiveToDormant`
- `GoDormant`
- `GoDormantToIdle`
- `GoIdle`
- `GoIdleToActive`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `PlaceEffectsInWorld`
- `PlayAnimationAndFX`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `SetFixedItemState` `V`
- `StartActiveEffect`
- `StartActiveFXMesh`
- `StartIdleEffect`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemShrine` `V`

### `FixedItemShrineController` (Game.dll, 11)

- `AnimationCallback` `V`
- `FixedItemShrineController`
- `FixedItemShrineController`
- `InitialUpdate` `V`
- `LoadFromDatabase` `V`
- `TouchedByActor` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `operator=`
- `~FixedItemShrineController` `V`

### `FixedItemSkill` (Game.dll, 16)

- `AssignParent` `V`
- `FixedItemSkill`
- `FixedItemSkill`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargets` `VC`
- `Load` `V`
- `PlayActiveFx`
- `PlayWarningFx`
- `PreLoad`
- `ProcessTargetResults` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~FixedItemSkill` `V`

### `FixedItemSkillTargetConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `FixedItemSkillTargetConfigCmd`
- `FixedItemSkillTargetConfigCmd`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~FixedItemSkillTargetConfigCmd` `V`

### `FixedItemSkillTargetConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FixedItemSkillTargetConfigCmdPacket`
- `FixedItemSkillTargetConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FixedItemSkillTargetConfigCmdPacket` `V`

### `FixedItemSkill_Buff` (Game.dll, 12)

- `FixedItemSkill_Buff`
- `FixedItemSkill_Buff`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargets` `VC`
- `Load` `V`
- `ProcessTargetResults` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~FixedItemSkill_Buff` `V`

### `FixedItemSkill_SpawnMonster` (Game.dll, 12)

- `FixedItemSkill_SpawnMonster`
- `FixedItemSkill_SpawnMonster`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargets` `VC`
- `Load` `V`
- `ProcessTargetResults` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~FixedItemSkill_SpawnMonster` `V`

### `FixedItemStateChangeConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `FixedItemStateChangeConfigCmd`
- `FixedItemStateChangeConfigCmd`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~FixedItemStateChangeConfigCmd` `V`

### `FixedItemStateChangeConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FixedItemStateChangeConfigCmdPacket`
- `FixedItemStateChangeConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FixedItemStateChangeConfigCmdPacket` `V`

### `FixedItemTargetConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `FixedItemTargetConfigCmd`
- `FixedItemTargetConfigCmd`
- `GetNetPacket` `V`
- ``vftable'`
- `operator=`
- `~FixedItemTargetConfigCmd` `V`

### `FixedItemTargetConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FixedItemTargetConfigCmdPacket`
- `FixedItemTargetConfigCmdPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FixedItemTargetConfigCmdPacket` `V`

### `FixedItemTeleport` (Game.dll, 26)

- `AnimationCallback` `V`
- `ApplyReplicationData` `V`
- `CreatePathObstacle` `V`
- `EnableOutline` `V`
- `FixedItemTeleport`
- `GetGameDescription` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RemovePathObstacle` `V`
- `RequestToUse` `V`
- `SetLock` `V`
- `SetOperation`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemTeleport` `V`

### `FixedItemTyphonPortal` (Game.dll, 20)

- `AnimationCallback` `V`
- `FixedItemTyphonPortal`
- `GetGameDescription` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsOfInterest` `VC`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `SetLock` `V`
- `SetOperation`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemTyphonPortal` `V`

### `FixedItemWell` (Game.dll, 12)

- `FixedItemWell`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `RTTI_new` `S`
- `SetFixedItemState` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FixedItemWell` `V`

### `FixedLever` (Game.dll, 36)

- `AnimationCallback` `V`
- `ApplyReplicationData` `V`
- `CanInteract` `C`
- `CanInteractWithEntity` `C`
- `Close` `V`
- `FixedLever`
- `GetGameDescription` `VC`
- `GetOpenCloseState` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsClosed` `C`
- `IsLocked` `C`
- `IsOfInterest` `VC`
- `IsOpen` `C`
- `Load` `V`
- `OnDestroy` `V`
- `Open` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `ResolveEnum_Interaction` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetAutoClose`
- `SetLocked`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kDefaultAutoClosePeriod` `S`
- `~FixedLever` `V`

### `FlameBeam` (Game.dll, 12)

- `FlameBeam`
- `Generate` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RenderPass` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FlameBeam` `V`

### `FlickerLight` (Engine.dll, 12)

- `AddToScene` `V`
- `FlickerLight`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FlickerLight` `V`

### `FloorProgressToClientPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FloorProgressToClientPacket`
- `FloorProgressToClientPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FloorProgressToClientPacket` `V`

### `FlyingBolt` (Engine.dll, 12)

- `FlyingBolt`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FlyingBolt` `V`

### `FogOfWar` (Engine.dll, 24)

- `AddVisibility`
- `CreateTexture`
- `Destroy`
- `DestroyTexture`
- `FogOfWar`
- `GetData`
- `GetTexture`
- `GetXSize` `C`
- `GetYSize` `C`
- `Initialize`
- `IsInFog` `C`
- `PostDeviceReset`
- `PreDeviceReset`
- `Read`
- `SetDirty`
- `UpdateTexture`
- `Write` `C`
- ``vftable'`
- `kInvScale` `S`
- `kScale` `S`
- `kVersion` `S`
- `kVisibleRange` `S`
- `operator==`
- `~FogOfWar` `V`

### `FogSectorData` (Engine.dll, 8)

- `Copy` `V`
- `FogSectorData`
- `FogSectorData`
- `FogSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~FogSectorData` `V`

### `Font` (Widget.dll, 4)

- `CreatePointFont`
- `Font`
- `operator HFONT__ * __ptr64`
- `~Font` `V`

### `FontStyle` (Engine.dll, 7)

- `FontStyle`
- `FontStyle`
- `FontStyle`
- `GetLineHeight` `C`
- `operator=`
- `operator=`
- `~FontStyle`

### `FragmentPacket` (Engine.dll, 9)

- `CopyInbound` `V`
- `FragmentPacket`
- `FragmentPacket`
- `GetOverhead`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FragmentPacket` `V`

### `FrameCounter` (Engine.dll, 4)

- `FrameCounter`
- `operator=`
- `operator=`
- `operator=`

### `FriendList` (Engine.dll, 5)

- `FriendList`
- `GetFriendByIndex`
- `GetFriendCount`
- `Refresh` `V`
- `~FriendList` `V`

### `FrustumUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `FrustumUpdatePacket`
- `FrustumUpdatePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~FrustumUpdatePacket` `V`

### `FusedBone` (Engine.dll, 6)

- `FusedBone`
- `FusedBone`
- `FusedBone`
- `operator=`
- `operator=`
- `~FusedBone`

### `Fx` (Game.dll, 26)

- `AddToWorld` `V`
- `AnimationCallback` `V`
- `Fx`
- `GetIntersection` `VC`
- `GetMyRegionCoords` `VC`
- `GetMyWorldCoords` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HandleAnimationCallbacks`
- `InitialUpdate` `V`
- `IsComplete` `VC`
- `Load` `V`
- `LoadResources` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RegisterAnimationCallback` `V`
- `SetParent` `V`
- `Start` `V`
- `Stop` `V`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Fx` `V`

### `FxMesh` (Game.dll, 34)

- `AddAnimation`
- `AddToScene` `V`
- `AddToWorld` `V`
- `AnimationCallback` `V`
- `CreateAttachmentsFromMesh` `V`
- `EndAnimation` `V`
- `FxMesh`
- `GetAttachPoint` `C`
- `GetAttachedCoords` `VC`
- `GetAttachedCoordsInRegion` `VC`
- `GetBaseScale` `C`
- `GetCurrentScale` `C`
- `GetRTTIClassInfo` `VC`
- `GetRandomIndex`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `LoadMeleeAnimation`
- `LoadRangedAnimation`
- `LoadResources` `V`
- `PlaySyncedAnimation` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `SetScale` `V`
- `SetScale` `V`
- `Start` `V`
- `Stop` `V`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FxMesh` `V`

### `FxPak` (Game.dll, 17)

- `DELAY_BEFORE_DESTROY_MS` `S`
- `DisableShake`
- `FxPak`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnAddToLevel` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `SetPlayerId` `V`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~FxPak` `V`

### `GAME` (Engine.dll, 86)

- `CharacterActionPermissionToString`
- `CharacterActionTypeToString`
- `Crc32`
- `Draw2DDirectionalQuad`
- `Draw2DOrientedQuad`
- `Draw2DRectangle`
- `Draw2DRectangle`
- `Draw2DRectangle`
- `DrawArc`
- `DrawArrow`
- `DrawAxisLines`
- `DrawAxisSphere`
- `DrawAxisSphere`
- `DrawCapsule`
- `DrawSolidBox`
- `DrawSolidBox`
- `DrawSolidCircle`
- `DrawSolidTetrahedron`
- `DrawWireTetrahedron`
- `DrawWireframeBox`
- `DrawWireframeBox`
- `DrawWireframeCircle`
- `ElapsedMilliseconds`
- `FormatNetworkErrorMessage`
- `GetErrorAsText`
- `GetGameTime`
- `GetMachineClockTicks`
- `GetMachineClockTicksPerSecond`
- `GetMachineTime`
- `GetPacketTypeAsText`
- `GetPacketTypeIsExcludedFromLog`
- `GetPerformanceCounter`
- `GetSystemTime`
- `GetTimeScale`
- `GetUnixTime`
- `GraphicsAlign`
- `GraphicsAlign`
- `GraphicsAlign`
- `IsGameTimePaused`
- `Lerp`
- `Lerp`
- `NetworkIndexToString`
- `PauseGameTime`
- `RIFFGetChunk`
- `RIFFGetWAVData`
- `SetTimeScale`
- `StringToNetworkIndex`
- `UnpauseGameTime`
- `UpdateTime`
- `gEngine`
- `gLogEngine`
- `gLogGraphics`
- `gLogNoCategory`
- `gLogRelease`
- `kHeadingDroppedItemByType`
- `kHeadingFrameStats`
- `kHeadingGame`
- `kHeadingItem`
- `kHeadingItemsCreated`
- `kHeadingKilledMonsterByType`
- `kHeadingMap`
- `kHeadingMonster`
- `kHeadingMonsterSkill`
- `kHeadingMonsterSkillUsedByType`
- `kHeadingPathing`
- `kHeadingPets`
- `kHeadingPetsReleased`
- `kHeadingPickedUpItemByType`
- `kHeadingPlayer`
- `kHeadingPlayerItems`
- `kHeadingPlayerSessionEnd`
- `kHeadingPlayerSessionStart`
- `kHeadingPlayerSkill`
- `kHeadingPlayerSkillUsedByType`
- `kHeadingPrefix`
- `kHeadingProxyStats`
- `kHeadingRelic`
- `kHeadingRelicUsedByType`
- `kHeadingSkills`
- `kHeadingSkillsUsedByType`
- `kHeadingSuffix`
- `kHeadingUsedItemByType`
- `operator!=`
- `operator*`
- `operator==`
- `operator==`

### `GAME` (Game.dll, 3)

- `GameTextLineToString`
- `gGameEngine`
- `gLogCombat`

### `GAME` (Widget.dll, 4)

- `CheckMenuItem`
- `EnableMenuItem`
- `IsMenuItemChecked`
- `MergeMenus`

### `GMIActor` (Game.dll, 16)

- `CollisionCallback` `V`
- `GMIActor`
- `GetBoneName` `C`
- `IsComplete` `C`
- `Load` `V`
- `PhysicsSetup` `V`
- `PreLoad` `V`
- `SetBoneName`
- `SetLiberateVelocity`
- `SetLiberateVelocityRandom`
- `SetScaleFactor`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `~GMIActor` `V`

### `GMIActor_Icy` (Game.dll, 17)

- `BeginEffects` `V`
- `Finish`
- `GMIActor_Icy`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Liberate` `V`
- `Load` `V`
- `OnGMIEffectDestroy` `V`
- `OnGMIEffectFinished` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~GMIActor_Icy` `V`

### `GMIEffect` (Engine.dll, 6)

- `GMIEffect`
- `GMIEffect`
- `SetShaderParameters` `VC`
- ``vftable'`
- `operator=`
- `~GMIEffect` `V`

### `GMIEffectIcey` (Engine.dll, 14)

- `BeginFinishing` `V`
- `CleanParent` `V`
- `GMIEffectIcey`
- `GMIEffectIcey`
- `IsCompatible` `S`
- `RemoveListener` `V`
- `SetDeflateRate`
- `SetShaderParameters` `VC`
- `SetUniformScaleFactor`
- `Update` `V`
- ``vftable'`
- `kFadePeriod` `S`
- `operator=`
- `~GMIEffectIcey` `V`

### `GMIEffectListener` (Engine.dll, 6)

- `GMIEffectListener`
- `GMIEffectListener`
- `GMIEffectListener`
- ``vftable'`
- `operator=`
- `operator=`

### `GPDDefaultVertexBuilder` (Engine.dll, 8)

- `GPDDefaultVertexBuilder`
- `GPDDefaultVertexBuilder`
- `GPDDefaultVertexBuilder`
- `GetVertexDeclaration` `VC`
- `SetVertex` `VC`
- ``vftable'`
- `operator=`
- `operator=`

### `GPDTex2VertexBuilder` (Engine.dll, 7)

- `GPDTex2VertexBuilder`
- `GPDTex2VertexBuilder`
- `GetVertexDeclaration` `VC`
- `SetVertex` `VC`
- ``vftable'`
- `operator=`
- `~GPDTex2VertexBuilder`

### `GameActivity` (Game.dll, 2)

- `GameActivity`
- `~GameActivity` `V`

### `GameBalanceUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GameBalanceUpdatePacket`
- `GameBalanceUpdatePacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~GameBalanceUpdatePacket` `V`

### `GameCamera` (Game.dll, 28)

- `AdjustYaw`
- `ClampTargetOffset`
- `GameCamera`
- `GetDistanceMax` `C`
- `GetDistanceMin` `C`
- `GetPlayer`
- `GetTarget` `VC`
- `GetTransparencyFrustum` `C`
- `GetTransparencyHeight` `C`
- `LerpToDefaults`
- `ResetToDefaults`
- `ResetZoom`
- `SetCameraYaw` `V`
- `SetControllingCamera`
- `SetMovementExtents`
- `SetPlayer`
- `SetTarget`
- `SetZoom`
- `Shake`
- `StopRotation`
- `Update` `V`
- `UpdateFromInputImpl` `V`
- `UpdatePitch`
- `UpdateTarget`
- `UpdateTransparencyFrustum`
- `YawLeft`
- `YawRight`
- `Zoom`

### `GameController` (Game.dll, 10)

- `Bind`
- `ConvertToJoystickEvent`
- `ConvertToMouseEvent` `C`
- `GameController`
- `GetActionSet` `C`
- `GetGlyphs` `C`
- `SetActionSet`
- `Update`
- `operator=`
- `~GameController`

### `GameEngine` (Game.dll, 809)

- `ActivateAltarToServer`
- `AddAscendantAltarToken`
- `AddAscendantDoubleRareMod`
- `AddAscendantEquippedItemsMod`
- `AddAscendantExperienceMod`
- `AddAscendantGoldMod`
- `AddAscendantMonsterTotemMod`
- `AddAscendantNemesisSpawnMod`
- `AddAscendantReagentMod`
- `AddAscendantReputationMod`
- `AddAscendantSuperBossMod`
- `AddAscendantSuperBossSpawnMod`
- `AddAscendantTreasureTroveMod`
- `AddChatMessage`
- `AddItemToFormulas`
- `AddItemToReagents`
- `AddItemToTransfer`
- `AddItemToTransfer`
- `AddItemToTransferRefund`
- `AddItemToTransmutes`
- `AddMutatorInbound`
- `AddMutatorOutbound`
- `AddPetBonusFxPak`
- `AddPlayerToParty`
- `AddPlayerToVideoList`
- `AddPotionUnlock`
- `AddTransferSack`
- `AddUINotification`
- `AddUINotification`
- `AllowRagdolls` `C`
- `AutoSave`
- `BackupFormulas`
- `BackupPlayerTransmutes`
- `BackupTransfer`
- `BanPlayer`
- `BoostFactionNegative`
- `BoostFactionPositive`
- `BroadcastMessageRemote`
- `CalculateAscensionReagentChance`
- `CalculateExperienceReward`
- `CalculateNewChanceToRun`
- `CalculateSuperBossChance`
- `CalculateTotemChance`
- `CalculateTroveChance`
- `CallEventHookCommand`
- `CallLuaQuestCommandEvent`
- `CallLuaQuestCommandGlobalEvent`
- `CanFactionBeNegativeBoosted`
- `CanFactionBePositiveBoosted`
- `CanRefundTransfer` `C`
- `CaravanGoodbye`
- `CharacterExperienceOutbound`
- `ChatCommandUseSkill`
- `CheckPlayerAction` `C`
- `CleanseShrineToServer`
- `ClearAllChallengeAreas`
- `ClearAltarFormulas`
- `ClearAwakenedFormulas`
- `ClearDroppedEpicsAndLegendaries`
- `ClearFormulas`
- `ClearMutatorsInbound`
- `ClearMutatorsOutbound`
- `ClearPetList`
- `ClearPotionUnlocks`
- `ClearReagents`
- `ClearTransferRefundList`
- `ClearTransmutes`
- `CloseMarket`
- `CompleteDifficulty`
- `CompleteSurvivalDifficulty`
- `CompleteSurvivalDifficulty`
- `ContributeMutatorCharAttributes` `C`
- `ContributeMutatorDefenseAttributes` `C`
- `ContributeMutatorOffensiveDamageAttributes` `C`
- `ContributeMutatorOffensiveModifierAttributes` `C`
- `ContributeMutatorRetaliationAttributes` `C`
- `ContributeMutatorRetaliationModifierAttributes` `C`
- `ContributeMutatorSkillAttributes` `C`
- `ContributePetBonusCharAttributes` `C`
- `ContributePetBonusConversionAttributes` `C`
- `ContributePetBonusDefenseAttributes` `C`
- `ContributePetBonusOffensiveDamageAttributes` `C`
- `ContributePetBonusOffensiveModifierAttributes` `C`
- `ContributePetBonusRetaliationAttributes` `C`
- `ContributePetBonusRetaliationModifierAttributes` `C`
- `ContributePetBonusSkillAttributes` `C`
- `ControlPlayerRespawnInbound`
- `ControlPlayerRespawnOutbound`
- `CreateEntity`
- `CreateFixedItemTeleport`
- `CreateFixedItemTeleport`
- `CreateFixedItemTeleportNetHook`
- `CreateGravestone`
- `CreateGravestone`
- `CreateGravestoneNetHook`
- `CreateItem`
- `CreateItemCopy`
- `CreateItemForCharacter`
- `CreateMarketClient`
- `CreateUIPlayerBuyText` `C`
- `CreateUIPlayerSellText` `C`
- `CreateUIPlayerTransferText`
- `CtoS_HandleNRLoaderStateChangeInbound`
- `CtoS_HandleNRLoaderStateChangeOutbound`
- `CtoS_InitiateServerRespawnLoaderOutbound`
- `CtoS_StartServerRespawnInbound`
- `CtoS_StartTeleportInbound`
- `CtoS_StartTeleportOutbound`
- `DebugCreateEntity`
- `DebugCreateItem`
- `DecActiveRagdolls`
- `DecrementNeedsSleepCounter`
- `DefaultSpawn`
- `DepositTransferReagents`
- `DestroyFixedItemTeleport`
- `DestroyFixedItemTeleportNetHook`
- `DestroyGravestone`
- `DestroyMarketClient`
- `DevotionPointsInUse`
- `DifficultyRampOutbound`
- `DisableEntityByUniqueId`
- `DisengageAltar`
- `DisengageContainer`
- `DisengageEndlessShrine`
- `DisengageNpc`
- `DisengageShrine`
- `DispelEndlessShrineBuffs`
- `DispelEndlessSoulBuffs`
- `DisplayCaravanWindow`
- `DisplayCrafterWindow`
- `DisplayEnchanterWindow`
- `DisplayItemAscensionWindow`
- `DisplayMessageRemote`
- `DisplayPotionsWindow`
- `DisplaySkillReallocationWindow`
- `DisplayTransmutesWindow`
- `DisplayWMessageRemote`
- `DropItemFloorTest`
- `DumpGroupProxyData`
- `DumpPetBonuses` `C`
- `EnableGameEngine`
- `EnableReagentsSave`
- `EnableTransferSave`
- `EnableTransmuteSave`
- `EngageAltarRequest`
- `EngageAltarResponse`
- `EngageContainerRequest`
- `EngageContainerResponse`
- `EngageEndlessShrineRequest`
- `EngageEndlessShrineResponse`
- `EngageNpcRequest`
- `EngageNpcResponse`
- `EngageShrineRequest`
- `EngageShrineResponse`
- `EventHookCommand`
- `ExitPlayingMode`
- `Expansion1MaxTransferSacks` `S`
- `Expansion2MaxTransferSacks` `S`
- `Expansion3MaxTransferSacks` `S`
- `FactionBoosterClamp` `C`
- `FactionClamp` `C`
- `FactionLevelToValue` `C`
- `FactionValueToLevel` `C`
- `FastSpawnEntity`
- `FillClassArray`
- `FilterInvalidTargets` `C`
- `FilterMutators`
- `FindAltarRecipe`
- `FindAltarRecipe`
- `FindNRLoader`
- `FindReagentId`
- `FindSuperBossSpawnPoint`
- `ForceMarketRefresh`
- `ForceRenderingEnabled`
- `GameEngine`
- `GameWon`
- `GameWonMsg`
- `GenerateEndlessDungeon`
- `GenerateUIDevotionSearchText` `S`
- `GenerateUIDevotionText` `S`
- `GenerateUIScrollSkillText` `S`
- `GenerateUISkillInfo` `S`
- `GenerateUISkillText` `S`
- `GetAchievementManager`
- `GetActivatedAltarReagents` `C`
- `GetAetherName` `C`
- `GetAffinityBitmap` `C`
- `GetAlertDistance` `C`
- `GetAllTargetsInRadius` `C`
- `GetAltarExclusiveRecipes` `C`
- `GetAltarInclusiveRecipes` `C`
- `GetArmorDefensiveAbsorption` `C`
- `GetAscendantAltarModifier` `C`
- `GetAscendantCharAttributes` `C`
- `GetAscendantDefenseAttributes` `C`
- `GetAscendantDoubleRareWeightMod` `C`
- `GetAscendantEquippedItemsMod` `C`
- `GetAscendantExperienceMod` `C`
- `GetAscendantGoldMod` `C`
- `GetAscendantLootTableOverride` `C`
- `GetAscendantMonsterTotemChance` `C`
- `GetAscendantNemesisSpawnMod` `C`
- `GetAscendantOffensiveDamageAttributes` `C`
- `GetAscendantOffensiveModifierAttributes` `C`
- `GetAscendantPrefixOverride` `C`
- `GetAscendantRarePrefixOverride` `C`
- `GetAscendantRareSuffixOverride` `C`
- `GetAscendantReputationMod` `C`
- `GetAscendantRetaliationAttributes` `C`
- `GetAscendantRetaliationModifierAttributes` `C`
- `GetAscendantSkillAttributes` `C`
- `GetAscendantSuffixOverride` `C`
- `GetAscendantSuperBossSpawnMod` `C`
- `GetAscendantTreasureTroveChance` `C`
- `GetAscendedItemSymbolName` `C`
- `GetAscensionReagentDropChance` `C`
- `GetAutoSave` `C`
- `GetAwakenedRecipes` `C`
- `GetBalanceDifficulty` `C`
- `GetBaseFolder` `S`
- `GetBonusGainEndMonsterTypes`
- `GetBossRange` `C`
- `GetCamera`
- `GetCaravanDriverTag` `C`
- `GetChallengeAdjustment` `C`
- `GetChallengeArea`
- `GetChestLootWeightModifiers` `C`
- `GetClassTag` `C`
- `GetClassTag` `C`
- `GetClientServices`
- `GetClientServices` `C`
- `GetCloudStorage` `C`
- `GetCombatIdleTime` `C`
- `GetComboChargeAuraName` `C`
- `GetComboChargeMultiplier` `C`
- `GetComboChargeTime` `C`
- `GetCompatibleReagent`
- `GetCompatibleTransfer`
- `GetCritTextStyle` `C`
- `GetDamageByAttacker` `C`
- `GetDatabase` `C`
- `GetDebugAttachpoints` `C`
- `GetDebugDestructibles` `C`
- `GetDebugItems` `C`
- `GetDebugMonsters` `C`
- `GetDebugProjectiles` `C`
- `GetDebugProxies` `C`
- `GetDebugQuestTokens` `C`
- `GetDebugSetpieces` `C`
- `GetDebugTransparency` `C`
- `GetDefaultLoadDistance` `C`
- `GetDefaultSkillManager`
- `GetDefaultSkillManager` `C`
- `GetDemoPeriod` `C`
- `GetDetailMapData`
- `GetDialogManager`
- `GetDifficultyFolder` `C`
- `GetDifficultyFromName` `C`
- `GetDifficultyRamp` `C`
- `GetDoubleRareMonsterInfrequentSymbolName` `C`
- `GetDoubleRareSymbolName` `C`
- `GetDualWieldBonusFactor` `C`
- `GetDualWieldSpeedFactor` `C`
- `GetDungeonGenerator` `C`
- `GetDungeonsGenerated` `C`
- `GetDurationDamageV` `C`
- `GetDynamiteName` `C`
- `GetEndlessDungeonId` `C`
- `GetEnemies`
- `GetEnemyOutlineColor`
- `GetEntityUnificationIndex`
- `GetEventBeginSound`
- `GetEventEndSound`
- `GetEventFailSound`
- `GetEventNpcSound`
- `GetEventNpcSound2`
- `GetFactionBountyEnabled`
- `GetFactionDiscountMultiplier`
- `GetFactionIcon` `C`
- `GetFactionLargeIcon` `C`
- `GetFactionLevelBounds` `C`
- `GetFactionLevelName` `C`
- `GetFactionLevelTag` `C`
- `GetFactionLevelValue`
- `GetFactionManager`
- `GetFactionQuestEnabled`
- `GetFactionTierReward` `C`
- `GetFriendOutlineColor`
- `GetFrustumForPlayer` `C`
- `GetFrustumForPlayer` `C`
- `GetFullSaveFolder` `C`
- `GetGameController`
- `GetGameDifficulty` `C`
- `GetGameDifficultyAsPath` `C`
- `GetGameMode` `C`
- `GetGameSettingsFolder` `S`
- `GetGameTextStyleIndent` `C`
- `GetGameTextStyleName` `C`
- `GetGeneralPurposeRandomSeed` `C`
- `GetGraveOwner` `C`
- `GetHostPlayer`
- `GetInboundNetworkInterface`
- `GetInputMode` `C`
- `GetInventoryCellSize`
- `GetInventorySackDims` `C`
- `GetItemBackground` `C`
- `GetItemBeamEffectNames`
- `GetItemClassificationName` `C`
- `GetItemColor` `C`
- `GetItemColorText` `C`
- `GetItemDropSoundName` `C`
- `GetItemHandsAnimationName` `C`
- `GetItemHighlightColor` `C`
- `GetItemMapNugget` `C`
- `GetItemMaxStackSize` `C`
- `GetItemSet` `C`
- `GetItemSparkleName` `C`
- `GetItemSparkleTimes` `C`
- `GetItemTextClass` `C`
- `GetItemTransferCost`
- `GetKillAllMonstersOnNextRespawn`
- `GetKillPenaltyEndMonsterTypes`
- `GetLastUsedTeleportId` `C`
- `GetLevelLimits` `C`
- `GetLoadDistance` `C`
- `GetLoadSphereForPlayer` `C`
- `GetLoadSphereForPlayer` `C`
- `GetLoadingTip` `C`
- `GetLoadingTipStyle` `C`
- `GetLocalPetList` `C`
- `GetLongRange` `C`
- `GetLootMode` `C`
- `GetMainPlayer` `C`
- `GetMainPlayersGraveData` `C`
- `GetMainPlayersPersonalPortalZone` `C`
- `GetMapFolder` `C`
- `GetMapFolder` `S`
- `GetMapName`
- `GetMarketInventorySack`
- `GetMarketInventorySack` `C`
- `GetMarketIsItemAffordable` `C`
- `GetMarketItemStatus` `C`
- `GetMarketMatchesFactionLevel` `C`
- `GetMasterAttacker` `C`
- `GetMaxAbsFxScale` `C`
- `GetMaxBoosterValue` `C`
- `GetMaxPlayerRotationSpeed` `C`
- `GetMaxReputationValue` `C`
- `GetMaxRotationSpeed` `C`
- `GetMaximumRange` `C`
- `GetMaximumSacks` `C`
- `GetMeleeAutoTargetDistance` `C`
- `GetMeleeRange` `C`
- `GetMeleeTargetDistance` `C`
- `GetMinDamageDebugging` `C`
- `GetMinPlayerRotationSpeed` `C`
- `GetMinRotationSpeed` `C`
- `GetMindControlledOutlineColor`
- `GetMiniPetLimit` `C`
- `GetModBaseFolder` `S`
- `GetModerateRange` `C`
- `GetMonsterAttributePak` `C`
- `GetMonsterColorText` `C`
- `GetMonsterInfrequentSymbolName` `C`
- `GetMonsterLootWeightModifiers` `C`
- `GetMonsterRaceTagPlural` `C`
- `GetMonsterRaceTagSingular` `C`
- `GetMutators`
- `GetNemesisKillCount` `C`
- `GetNemesisRespawnKillModifier` `C`
- `GetNetworkInterface`
- `GetNewCharacterTimeOfDay` `C`
- `GetNullSkillResource` `C`
- `GetNumFactionLevels` `C`
- `GetNumNotifications`
- `GetNumPartyRequestResponses` `C`
- `GetObjectOutlineColor`
- `GetObjectives`
- `GetOverlayColor`
- `GetPartyManager`
- `GetPathSlowdownLength` `C`
- `GetPetAttributePak` `C`
- `GetPetBonusCharFxPakNames` `C`
- `GetPetBonusList` `C`
- `GetPetBonusPotionScale` `C`
- `GetPlayerAttributePak` `C`
- `GetPlayerBaseAttackSpeed` `C`
- `GetPlayerCompletedLevel` `C`
- `GetPlayerDamagePercent` `C`
- `GetPlayerDeathExperiencePenalty` `C`
- `GetPlayerExperienceRedemptionAmount`
- `GetPlayerFolder` `C`
- `GetPlayerFolder` `C`
- `GetPlayerFormulas` `C`
- `GetPlayerId` `C`
- `GetPlayerInfo` `C`
- `GetPlayerManagerClient` `C`
- `GetPlayerManagerServer` `C`
- `GetPlayerName` `C`
- `GetPlayerPathSlowdownLength` `C`
- `GetPlayerReagents` `C`
- `GetPlayerRespawn` `C`
- `GetPlayerTransfer`
- `GetPlayerTransmutes` `C`
- `GetPlayerUnlockedLevel` `C`
- `GetPotionModifierComboBitmap` `C`
- `GetPreAwakenedItemSymbolName` `C`
- `GetProxyTable` `C`
- `GetPuritySkills` `C`
- `GetPvpCrowdControlDurationMultiplier` `C`
- `GetPvpDamageMultiplier` `C`
- `GetRandomGen`
- `GetRandomSeed`
- `GetRangedComboChargeTime` `C`
- `GetReagentItemCount` `C`
- `GetRootSavePath` `S`
- `GetSelectedTransferSackNumber` `C`
- `GetSelectionBiasBaseMultiplier` `C`
- `GetSelectionBiasBaseOffset` `C`
- `GetSelectionBiasComparisonMultiplier` `C`
- `GetSelectionBiasComparisonOffset` `C`
- `GetSelectionBiasLockOnMultiplier` `C`
- `GetSelectionBiasLockOnOffset` `C`
- `GetSelectionBiasMouseDownOffset` `C`
- `GetSelectionBiasVelocityMultiplier` `C`
- `GetServerServices`
- `GetServerServices` `C`
- `GetSharedSavePath` `C`
- `GetShortRange` `C`
- `GetShowInstancedItems` `C`
- `GetSimInformation`
- `GetSkillMasteryLevel` `C`
- `GetSkillMasteryTierLevels` `C`
- `GetSkillResource` `C`
- `GetSuperBossChance` `C`
- `GetSuperBossSpawnKills`
- `GetTargetsAlongRay` `C`
- `GetTargetsAroundRay` `C`
- `GetTargetsInCone` `C`
- `GetTargetsInFrustum` `C`
- `GetTargetsInRadius` `C`
- `GetTargetsInRadius` `C`
- `GetTeleportInfo`
- `GetTeleportInfoList` `C`
- `GetTopPartyRequestResponse`
- `GetTotalDamageDone` `C`
- `GetTradeManager`
- `GetTransferItemCount` `C`
- `GetTransferSack`
- `GetTransparencyFrustumParams` `C`
- `GetTreasureRoomSound`
- `GetTutorialUnlockList`
- `GetTwoHandedBonusFactor` `C`
- `GetUGCharacterHighlight` `C`
- `GetUI` `C`
- `GetUpdateSphereForPlayer` `C`
- `GetUserSaveDataFolder` `C`
- `GetUserSettingsFolder` `S`
- `GetVoiceChatManager`
- `GetWaveEndSound`
- `GiveAscendedItemToItemAscensionWindow`
- `GiveDismantledBonusItemToEnchanterWindow`
- `GiveDismantledItemToEnchanterWindow`
- `GiveRecoveredItemToEnchanterWindow`
- `GiveRerollItemToEnchanterWindow`
- `GiveSetItemToEnchanterWindow`
- `GiveTinkeredItemToEnchanterWindow`
- `HandleActivateAltarToServerInbound`
- `HandleAltarReagentsInbound`
- `HandleAltarReagentsRequestInbound`
- `HandleAltarReagentsRequestOutbound`
- `HandleBonusToServerInbound`
- `HandleBonusToServerOutbound`
- `HandleCleanseShrineToServerInbound`
- `HandleDungeonFloorProgressToClientInbound`
- `HandleDungeonFloorProgressToClientOutbound`
- `HandleDungeonOpenExitInbound`
- `HandleDungeonOpenExitOutbound`
- `HandleDungeonProgressRequestInbound`
- `HandleDungeonProgressRequestOutbound`
- `HandleDungeonProgressToClientInbound`
- `HandleDungeonProgressToClientOutbound`
- `HandleDungeonProgressToServerInbound`
- `HandleDungeonProgressToServerOutbound`
- `HandleDungeonSoundInbound`
- `HandleDungeonSoundOutbound`
- `HandleExperienceNotification`
- `HandleFactionToServerInbound`
- `HandleInboundInspectHelper`
- `HandleInspectRequest`
- `HandleMutatorRequestInbound`
- `HandleMutatorRequestOutbound`
- `HandleShrineReward`
- `HandleStartShrineProxyToServerInbound`
- `HandleSurvivalRequestInbound`
- `HandleSurvivalRequestOutbound`
- `HasDroppedEpicOrLegendaryItem` `C`
- `HasFixedItemTeleport`
- `HasPotionUnlock`
- `ImplantStatisticRunGameInfo`
- `IncActiveRagdolls`
- `IncrementNeedsSleepCounter`
- `Initialize`
- `InitializeActionMatrix`
- `InitializeAssertHandler`
- `InitializeMonsterRaceMap`
- `InitializeRouterServices`
- `InitiatePlayerTeleport`
- `InspectPlayer`
- `InstallPetBonus`
- `InstallTeleportActivity`
- `InvitePlayerToParty`
- `IsAliveOrIntact` `C`
- `IsAttackableClass` `C`
- `IsAwakenedItem` `C`
- `IsBackupPlayerFolder` `S`
- `IsCharacterInNonPvpArea` `C`
- `IsDeathLoaderActive` `C`
- `IsDungeonBonusComplete` `C`
- `IsDungeonBossDead` `C`
- `IsFactionNegativeBoosted`
- `IsFactionPlayerVisible`
- `IsFactionPositiveBoosted`
- `IsGameEngineOnline` `C`
- `IsGameLoading` `C`
- `IsGameWaiting`
- `IsHiddenFaction` `C`
- `IsItemInFormulas` `C`
- `IsItemInTransfer` `C`
- `IsLeftTriggerDown` `C`
- `IsLocalPet` `C`
- `IsMouseLocked` `C`
- `IsNewTransmuteFile` `C`
- `IsNpcTalkingAllowed` `C`
- `IsPlayerAlive`
- `IsPlayingVideo` `C`
- `IsPreAwakenedItem` `C`
- `IsProxyHeroBossInGame` `C`
- `IsRenderingEnabled` `C`
- `IsReputationGainPossible` `C`
- `IsReputationGainPossibleIfNegative` `C`
- `IsRightTriggerDown` `C`
- `IsSaveEnabled` `C`
- `IsServerOrSingle` `C`
- `IsStartingFaction` `C`
- `IsTransferOpen` `C`
- `IsUltimateVeteran` `C`
- `IsUniqueEntitySelected`
- `IsViewDistanceLocked` `C`
- `KillAllMonsters`
- `LoadAscendantBossBonuses`
- `LoadAscendantMonsterBonuses`
- `LoadAscendantNonBossBonuses`
- `LoadFromDatabase`
- `LoadItemSet`
- `LoadPlayerFormulas`
- `LoadPlayerReagents`
- `LoadPlayerTransfer`
- `LoadPlayerTransmutes`
- `LoadPotionUnlocks`
- `LoadQuestStatesFromFile`
- `LoadSkillResource`
- `LoadSurvivalScore`
- `LoadTransferTransmutes`
- `LockKeyMapEvents`
- `LockMouse`
- `LogDroppedEpicsAndLegendaries`
- `LogMarkets`
- `LogPlayerCodex`
- `LogPlayerConversations`
- `LogPlayerFormulas`
- `LogPlayerInventory`
- `LogPlayerStash`
- `LogPlayerTransfer`
- `LogRouterStatus`
- `LuaCommandGlobalEvent`
- `MainPlayerCanUseAscendantAltar` `C`
- `MainPlayerCanUseConvert` `C`
- `MainPlayerCanUseDismantle` `C`
- `MainPlayerCanUsePersonalTeleport` `C`
- `MainPlayerCanUseReroll` `C`
- `MainPlayerHasPersonalTeleport` `C`
- `MangleBackupPlayerName` `S`
- `ManglePlayerName` `S`
- `MaxLeaderboardSize` `S`
- `MaxTransferSacks` `S`
- `MonsterUseController`
- `NeedsSleep` `C`
- `NemesisSpawn`
- `NemesisSpawnToServer`
- `NotifyCrafterWindow`
- `NotifyNewMutators`
- `NpcTalkInbound`
- `NpcTalkOutbound`
- `OnCreatureDeath`
- `OpenMarket`
- `PickObject`
- `PlayErrorSound`
- `PlayLearnFormulaSound`
- `PlayMarketPurchaseSound`
- `PlayMarketSaleSound`
- `PlayMutatorPlayerFx`
- `PlayMutatorSound`
- `PlayVideo`
- `PlayVideoCommand`
- `PlayVideoRequest`
- `PlayerExperienceRedemption`
- `PlayerPurchaseRequest`
- `PlayerSaleRequest`
- `PostLuaInitialize`
- `PostPetSpawn`
- `PreLuaShutdown`
- `PurgePetBonus`
- `QuestCommandBeginQuestTask`
- `QuestCommandCompleteQuest`
- `QuestCommandCompleteQuestTask`
- `QuestCommandDeclareTokens`
- `QuestCommandDestroyDestructible`
- `QuestCommandEnableMonsterSkills`
- `QuestCommandEvent`
- `QuestCommandGiveToken`
- `QuestCommandGlobalEvent`
- `QuestCommandLockChest`
- `QuestCommandLockDoor`
- `QuestCommandMove`
- `QuestCommandOpenDoor`
- `QuestCommandOpenDynGridEntrance`
- `QuestCommandPlayAnimation`
- `QuestCommandRemoveToken`
- `QuestCommandTakeItem`
- `QuestCommandToggleInvincible`
- `QuestCommandUiNotify`
- `QuestCommandUseSkill`
- `QuestCommandWalk`
- `ReadPlayerReagents`
- `ReadPlayerTransfer`
- `ReadPlayerTransmutes`
- `ReadPotionUnlocks`
- `RegisterDamage`
- `RegisterEpicOrLegendaryItem`
- `RegisterExperienceLoss`
- `RegisterLocalPet`
- `RegisterNRLoader`
- `RegisterPartyRequestResponse`
- `RegisterProxyHeroBoss`
- `ReloadDatabase`
- `RemoveAllPetBonuses`
- `RemoveDamageEntry`
- `RemoveGravestone`
- `RemoveGravestoneFor`
- `RemoveGravestoneInDungeon`
- `RemoveItemFromFormulas`
- `RemoveItemFromFormulas`
- `RemoveItemFromTransfer`
- `RemoveItemFromTransferRefund`
- `RemoveItemFromTransferSacks`
- `RemovePendingMessages`
- `RemovePetBonus`
- `RemovePetBonusFxPak`
- `RemovePlayerFromGame`
- `RemovePlayerFromVideoList`
- `RemovePortalsFor`
- `RenamePlayer`
- `ResetDungeonsGenerated`
- `ResetPlayerConversations`
- `ResetPlayerDeaths`
- `ResetPlayerFactions`
- `ResetTutorialState`
- `ResetUIKeyToggles`
- `ResolveChestClassificationEnum` `S`
- `ResolveEquationVariable` `VC`
- `ResolveItemClassificationEnum` `S`
- `ResolveMonsterClassificationEnum` `S`
- `RestoreNumberOfTransferSacks`
- `SaveFormulas`
- `SaveGame`
- `SaveGame`
- `SavePotionUnlocks`
- `SaveQuestStatesToFile`
- `SaveReagents`
- `SaveSurvivalScore`
- `SaveTransferStash`
- `SaveTransmutes`
- `ScriptEvent`
- `SelectSingleEntityFromGroup`
- `SendChatMessage`
- `SendFixedItemGravestones`
- `SendFixedItemTeleports`
- `SendInspectRequest`
- `SendPartyRequestResponse`
- `SendQuestPacket`
- `SendVoiceChat`
- `ServerPropegateTeleport`
- `SetAsNetworkClient`
- `SetAsNetworkServer`
- `SetAsNetworkStub`
- `SetAscendantLootTableOverride`
- `SetAscendantPrefixOverride`
- `SetAscendantRarePrefixOverride`
- `SetAscendantRareSuffixOverride`
- `SetAscendantSuffixOverride`
- `SetAssertHandlerLogPath`
- `SetAutoSave`
- `SetCameraFOV`
- `SetCaravanDriverTag`
- `SetClassEntry`
- `SetCloudStorage`
- `SetDebugAttachpoints`
- `SetDebugDestructibles`
- `SetDebugInputs`
- `SetDebugItems`
- `SetDebugMonsters`
- `SetDebugProjectiles`
- `SetDebugProxies`
- `SetDebugQuestTokens`
- `SetDebugSetpieces`
- `SetDebugTransparency`
- `SetDemoPeriod`
- `SetDifficultyRamp`
- `SetEndlessTimerInbound`
- `SetEndlessTimerOutbound`
- `SetEntityUnificationIndex`
- `SetGameBalanceLevel`
- `SetGameDifficulty`
- `SetGameLoading`
- `SetGameMode`
- `SetGeneralPurposeRandomSeed`
- `SetInputMode`
- `SetKillAllMonstersOnNextRespawn`
- `SetLastUsedTeleportId`
- `SetLeftTrigger`
- `SetLuaGlueLogging`
- `SetMarketItemCostStored`
- `SetMinDamageDebugging`
- `SetOverlayColor`
- `SetPetBonus`
- `SetPlayer`
- `SetPlayerById`
- `SetPlayerCompletedLevel`
- `SetPlayerSkinColor`
- `SetPlayerUnlockedLevel`
- `SetRenderingEnabled`
- `SetRightTrigger`
- `SetSaveEnabled`
- `SetSelectedTransferSackNumber`
- `SetShowInstancedItems`
- `SetSurvivalModeData`
- `SetSurvivalRestartsInbound`
- `SetSurvivalRestartsOutbound`
- `SetSurvivalTimerInbound`
- `SetSurvivalTimerOutbound`
- `SetSurvivalWaveTierInbound`
- `SetSurvivalWaveTierOutbound`
- `SetTransferOpen`
- `SetTransparencyFrustumParams`
- `SetUI`
- `SetViewDistance`
- `ShakeCamera`
- `ShowEntity`
- `ShowGlowEffect`
- `ShowTutorialPage`
- `Shutdown`
- `StartRespawnForLocalPlayer`
- `StartShrineProxyToServer`
- `StoC_HandleNRLoaderStateChangeInbound`
- `StoC_HandleNRLoaderStateChangeOutbound`
- `StoC_StartTeleportInbound`
- `StoC_StartTeleportOutbound`
- `StoH_StartTeleportOutbound`
- `StopVideo`
- `StopVideoRequest`
- `StreamPlayerFormulas`
- `SuperBossNotificationInbound`
- `SuperBossNotificationOutbound`
- `SuperPreRun`
- `SyncCaravanReagents`
- `SyncDungeonProgress`
- `SyncObjectivesInbound`
- `SyncObjectivesOutbound`
- `TakeItemFromReagents`
- `TakeItemFromReagents`
- `TakeItemFromTransfer`
- `TakeItemFromTransfer`
- `TakeTopNotification`
- `TryJump`
- `UIExists` `C`
- `UnJoinLeader`
- `UnJoinLeader_Net`
- `UninstallPetBonus`
- `UnloadWorld`
- `UnlockAchievement`
- `UnlockAllTutorials`
- `UnlockKeyMapEvents`
- `UnlockMouse`
- `UnlockTutorialPage`
- `UnregisterLocalPet`
- `UnregisterNRLoader`
- `UnregisterProxyHeroBoss`
- `Update`
- `ValidateArtifactPortal`
- `ValidateGravestone`
- `WritePlayerReagents` `C`
- `WritePlayerTransfer` `C`
- `WritePlayerTransmutes` `C`
- `WritePotionUnlocks` `C`
- ``vftable'`
- `kMaxActiveRagdolls` `S`
- `kMutatorEffectDelay` `S`
- `~GameEngine`

### `GameEngineInboundInterface` (Game.dll, 119)

- `AddMutator` `V`
- `AddPetBonusFxPak` `V`
- `AddPlayerToParty` `V`
- `C2S_MarketPurchaseRequest` `V`
- `C2S_MarketSellBack` `V`
- `C2S_MarketUpdateRequest` `V`
- `ChatCommandUseSkill` `V`
- `ClearMutators` `V`
- `ControlPlayerRespawn` `V`
- `CreateEntity` `V`
- `CreateFixedItemTeleport` `V`
- `CreateGravestone` `V`
- `CreateItem` `V`
- `CtoS_AddNewPlayer` `V`
- `CtoS_SinglePlayerHeartbeat` `V`
- `DestroyFixedItemTeleport` `V`
- `DestroyGravestone` `V`
- `DifficultyRampUpdate` `V`
- `DisengageAltar` `V`
- `DisengageContainer` `V`
- `DisengageEndlessShrine` `V`
- `DisengageNpc` `V`
- `DisengageShrine` `V`
- `DisplayMessageRemote` `V`
- `DisplayWMessageRemote` `V`
- `EngageAltarRequest` `V`
- `EngageAltarResponse` `V`
- `EngageContainerRequest` `V`
- `EngageContainerResponse` `V`
- `EngageEndlessShrineRequest` `V`
- `EngageEndlessShrineResponse` `V`
- `EngageNpcRequest` `V`
- `EngageNpcResponse` `V`
- `EngageShrineRequest` `V`
- `EngageShrineResponse` `V`
- `EventHookCommand` `V`
- `GameBalanceUpdate` `V`
- `GameEngineInboundInterface`
- `GameEngineInboundInterface`
- `GameWonMsg` `V`
- `GiveGoldToPlayer` `V`
- `HandleActivateAltar` `V`
- `HandleAltarReagents` `V`
- `HandleAltarReagentsRequest` `V`
- `HandleBonusToClient` `V`
- `HandleBonusToServer` `V`
- `HandleChatMessage` `V`
- `HandleCleanseShrine` `V`
- `HandleDungeonFloorProgressToClient` `V`
- `HandleDungeonOpenExit` `V`
- `HandleDungeonProgressToClient` `V`
- `HandleDungeonProgressToServer` `V`
- `HandleDungeonSound` `V`
- `HandleEndlessDungeonRequest` `V`
- `HandleExperienceNotification` `V`
- `HandleFactionToClient` `V`
- `HandleMutatorRequest` `V`
- `HandleQuestMessagePacket` `V`
- `HandleShrineReward` `V`
- `HandleStartShrineProxy` `V`
- `HandleSurvivalRequest` `V`
- `HandleVoiceChat` `V`
- `InvitePlayerToParty` `V`
- `LuaCommandGlobalEvent` `V`
- `MarketCreateItem` `V`
- `MonsterUseController` `V`
- `NemesisSpawn` `V`
- `NpcTalk` `V`
- `PlayVideoCommand` `V`
- `PlayVideoRequest` `V`
- `PostPetSpawn` `V`
- `QuestCommandBeginQuestTask` `V`
- `QuestCommandCompleteQuest` `V`
- `QuestCommandCompleteQuestTask` `V`
- `QuestCommandDeclareTokens` `V`
- `QuestCommandDestroyDestructible` `V`
- `QuestCommandEnableMonsterSkills` `V`
- `QuestCommandEvent` `V`
- `QuestCommandGiveToken` `V`
- `QuestCommandGlobalEvent` `V`
- `QuestCommandLockChest` `V`
- `QuestCommandLockDoor` `V`
- `QuestCommandMove` `V`
- `QuestCommandOpenDoor` `V`
- `QuestCommandOpenDynGridEntrance` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandRemoveToken` `V`
- `QuestCommandTakeItem` `V`
- `QuestCommandUiNotify` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RemovePetBonus` `V`
- `RemovePetBonusFxPak` `V`
- `S2C_MarketItemStatusUpdate` `V`
- `S2C_MarketPurchaseConfirmation` `V`
- `S2C_MarketPurchaseDenied` `V`
- `S2C_MarketUpdate` `V`
- `SetEndlessTimer` `V`
- `SetPetBonus` `V`
- `SetSurvivalRestarts` `V`
- `SetSurvivalTimer` `V`
- `SetSurvivalWaveTier` `V`
- `ShowEntity` `V`
- `StoC_AllPlayersHeartbeat` `V`
- `StoC_AllPlayersUpdate` `V`
- `StopVideoRequest` `V`
- `SuperBossNotification` `V`
- `SurvivalModeData` `V`
- `SyncObjectives` `V`
- `TradeAddItem` `V`
- `TradeCancel` `V`
- `TradeFinalize` `V`
- `TradeInitiate` `V`
- `TradeRemoveItem` `V`
- `TradeSetGoldAmount` `V`
- `UnJoinLeader_Net` `V`
- ``vftable'`
- `operator=`
- `~GameEngineInboundInterface` `V`

### `GameEngineNetworkInterface` (Game.dll, 5)

- `GameEngineNetworkInterface`
- `GameEngineNetworkInterface`
- ``vftable'`
- `operator=`
- `~GameEngineNetworkInterface` `V`

### `GameEngineOutboundInterface` (Game.dll, 119)

- `AddMutator` `V`
- `AddPetBonusFxPak` `V`
- `AddPlayerToParty` `V`
- `C2S_MarketPurchaseRequest` `V`
- `C2S_MarketSellBack` `V`
- `C2S_MarketUpdateRequest` `V`
- `ChatCommandUseSkill` `V`
- `ClearMutators` `V`
- `ControlPlayerRespawn` `V`
- `CreateEntity` `V`
- `CreateFixedItemTeleport` `V`
- `CreateGravestone` `V`
- `CreateItem` `V`
- `CtoS_AddNewPlayer` `V`
- `CtoS_SinglePlayerHeartbeat` `V`
- `DestroyFixedItemTeleport` `V`
- `DestroyGravestone` `V`
- `DifficultyRampUpdate` `V`
- `DisengageAltar` `V`
- `DisengageContainer` `V`
- `DisengageEndlessShrine` `V`
- `DisengageNpc` `V`
- `DisengageShrine` `V`
- `DisplayMessageRemote` `V`
- `DisplayWMessageRemote` `V`
- `EngageAltarRequest` `V`
- `EngageAltarResponse` `V`
- `EngageContainerRequest` `V`
- `EngageContainerResponse` `V`
- `EngageEndlessShrineRequest` `V`
- `EngageEndlessShrineResponse` `V`
- `EngageNpcRequest` `V`
- `EngageNpcResponse` `V`
- `EngageShrineRequest` `V`
- `EngageShrineResponse` `V`
- `EventHookCommand` `V`
- `GameBalanceUpdate` `V`
- `GameEngineOutboundInterface`
- `GameEngineOutboundInterface`
- `GameWonMsg` `V`
- `GiveGoldToPlayer` `V`
- `HandleActivateAltar` `V`
- `HandleAltarReagents` `V`
- `HandleAltarReagentsRequest` `V`
- `HandleBonusToClient` `V`
- `HandleBonusToServer` `V`
- `HandleChatMessage` `V`
- `HandleCleanseShrine` `V`
- `HandleDungeonFloorProgressToClient` `V`
- `HandleDungeonOpenExit` `V`
- `HandleDungeonProgressToClient` `V`
- `HandleDungeonProgressToServer` `V`
- `HandleDungeonSound` `V`
- `HandleEndlessDungeonRequest` `V`
- `HandleExperienceNotification` `V`
- `HandleFactionToClient` `V`
- `HandleMutatorRequest` `V`
- `HandleQuestMessagePacket` `V`
- `HandleShrineReward` `V`
- `HandleStartShrineProxy` `V`
- `HandleSurvivalRequest` `V`
- `HandleVoiceChat` `V`
- `InvitePlayerToParty` `V`
- `LuaCommandGlobalEvent` `V`
- `MarketCreateItem` `V`
- `MonsterUseController` `V`
- `NemesisSpawn` `V`
- `NpcTalk` `V`
- `PlayVideoCommand` `V`
- `PlayVideoRequest` `V`
- `PostPetSpawn` `V`
- `QuestCommandBeginQuestTask` `V`
- `QuestCommandCompleteQuest` `V`
- `QuestCommandCompleteQuestTask` `V`
- `QuestCommandDeclareTokens` `V`
- `QuestCommandDestroyDestructible` `V`
- `QuestCommandEnableMonsterSkills` `V`
- `QuestCommandEvent` `V`
- `QuestCommandGiveToken` `V`
- `QuestCommandGlobalEvent` `V`
- `QuestCommandLockChest` `V`
- `QuestCommandLockDoor` `V`
- `QuestCommandMove` `V`
- `QuestCommandOpenDoor` `V`
- `QuestCommandOpenDynGridEntrance` `V`
- `QuestCommandPlayAnimation` `V`
- `QuestCommandRemoveToken` `V`
- `QuestCommandTakeItem` `V`
- `QuestCommandUiNotify` `V`
- `QuestCommandUseSkill` `V`
- `QuestCommandWalk` `V`
- `RemovePetBonus` `V`
- `RemovePetBonusFxPak` `V`
- `S2C_MarketItemStatusUpdate` `V`
- `S2C_MarketPurchaseConfirmation` `V`
- `S2C_MarketPurchaseDenied` `V`
- `S2C_MarketUpdate` `V`
- `SetEndlessTimer` `V`
- `SetPetBonus` `V`
- `SetSurvivalRestarts` `V`
- `SetSurvivalTimer` `V`
- `SetSurvivalWaveTier` `V`
- `ShowEntity` `V`
- `StoC_AllPlayersHeartbeat` `V`
- `StoC_AllPlayersUpdate` `V`
- `StopVideoRequest` `V`
- `SuperBossNotification` `V`
- `SurvivalModeData` `V`
- `SyncObjectives` `V`
- `TradeAddItem` `V`
- `TradeCancel` `V`
- `TradeFinalize` `V`
- `TradeInitiate` `V`
- `TradeRemoveItem` `V`
- `TradeSetGoldAmount` `V`
- `UnJoinLeader_Net` `V`
- ``vftable'`
- `operator=`
- `~GameEngineOutboundInterface` `V`

### `GameInfo` (Engine.dll, 78)

- `AddPlayer`
- `ClearPlayerList`
- `GameInfo`
- `GameInfo`
- `GetAutoParty` `C`
- `GetChallenge` `C`
- `GetDifficulty` `C`
- `GetEventInProgress` `C`
- `GetGameName` `C`
- `GetGameNameAsStr` `C`
- `GetGamePlusChallengeDifficulty` `C`
- `GetGameStartTime` `C`
- `GetGender` `C`
- `GetHardcore` `C`
- `GetHostName` `C`
- `GetInstancedLoot` `C`
- `GetIsBackup` `C`
- `GetIsMultiPlayer` `C`
- `GetIsPlayingAtLeastThisDifficulty` `C`
- `GetIsServer` `C`
- `GetIsSinglePlayer` `C`
- `GetJoinedGameIsPvP`
- `GetLevelName`
- `GetLevelName`
- `GetLevelRange` `C`
- `GetLocalMapPath`
- `GetMaxLevel` `C`
- `GetMaxPlayers`
- `GetMinLevel` `C`
- `GetModName`
- `GetModName` `C`
- `GetMode` `C`
- `GetNumOfPlayers` `C`
- `GetPassword` `C`
- `GetPlayerCinematicStringTag` `C`
- `GetPlayerFlagIndex` `C`
- `GetPlayerLevel` `C`
- `GetPlayerName`
- `GetPlayerName`
- `GetPlayers`
- `GetPvP`
- `GetSurvivalIndex` `C`
- `GetTunicColorIndex` `C`
- `RemovePlayer`
- `SetAutoParty`
- `SetChallenge`
- `SetDifficulty`
- `SetEventInProgress`
- `SetGameName`
- `SetGameNameFromStr`
- `SetGameStartTime`
- `SetGender`
- `SetHardcore`
- `SetHostName`
- `SetInstancedLoot`
- `SetIsBackup`
- `SetIsMultiPlayer`
- `SetIsServer`
- `SetJoinedGameIsPvP`
- `SetLevelName`
- `SetLevelName`
- `SetLevelRange`
- `SetLocalMapPath`
- `SetMaxPlayers`
- `SetModName`
- `SetModName`
- `SetMode`
- `SetNumOfPlayers`
- `SetPassword`
- `SetPlayerInfo`
- `SetPlayerInfo`
- `SetPlayerLevel`
- `SetPvP`
- `SetSurvivalIndex`
- `SetTunicColorIndex`
- ``vftable'`
- `operator=`
- `~GameInfo` `V`

### `GameTextLine` (Game.dll, 2)

- `GameTextLine`
- `GameTextLine`

### `GameTextString` (Game.dll, 3)

- `GameTextString`
- `GetIconHeight` `C`
- `GetIconWidth` `C`

### `GameTimer` (Engine.dll, 7)

- `GameTimer`
- `GameTimer`
- `GameTimer`
- `GetInternalTime` `VC`
- ``vftable'`
- `operator=`
- `operator=`

### `GameWonPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GameWonPacket`
- `GameWonPacket`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~GameWonPacket` `V`

### `GarmentManager` (Game.dll, 9)

- `CalculateAllocatedMemory` `C`
- `GarmentManager`
- `GarmentManager`
- `SetClothing` `V`
- `SetVestment` `V`
- `UpdateGarment` `V`
- ``vftable'`
- `operator=`
- `~GarmentManager` `V`

### `GibEffectEntity` (Game.dll, 14)

- `CanBePlacedInEditor` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GibEffectEntity`
- `InitialUpdate` `V`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~GibEffectEntity` `V`

### `GiveGoldPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `GiveGoldPacket`
- `GiveGoldPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~GiveGoldPacket` `V`

### `GlobalCounter` (Engine.dll, 4)

- `GlobalCounter`
- `operator=`
- `operator=`
- `operator=`

### `GoldGenerator` (Game.dll, 15)

- `GetGoldActor` `C`
- `GoldGenerator`
- `GoldGenerator`
- `LoadFromDatabase`
- `ResolveEquationVariable` `VC`
- `RunGenerator`
- `SetChance`
- `SetGeneratorLevel`
- `SetLootMode`
- `SetMonsterExperience`
- `SetPlayerLevel`
- `SplitGold` `C`
- ``vftable'`
- `operator=`
- `~GoldGenerator` `V`

### `GraphicsAnim` (Engine.dll, 28)

- `CacheCreateEntities` `C`
- `Destroy` `V`
- `GetAnimSpeed` `C`
- `GetBoneCoords` `C`
- `GetBoneName` `C`
- `GetCallbackPoint` `C`
- `GetCreateEntity` `C`
- `GetDefaultName` `S`
- `GetFramePose` `C`
- `GetFrameRate` `C`
- `GetLength` `C`
- `GetNumBones` `C`
- `GetNumCallbackPoints` `C`
- `GetNumCreateEntity` `C`
- `GetNumRemoveEntity` `C`
- `GetPose` `C`
- `GetPose` `C`
- `GetRagDollDirection` `C`
- `GetRagDollEffectType` `C`
- `GetRagDollElevation` `C`
- `GetRagDollPush` `C`
- `GetRemoveEntity` `C`
- `GetSystemMemoryUsage` `VC`
- `GraphicsAnim`
- `Initialize` `V`
- `LoadANMData`
- `ReleaseCreateEntities` `C`
- `~GraphicsAnim` `V`

### `GraphicsCanvas` (Engine.dll, 97)

- `ApplyAdjustment`
- `ApplyColorRemap`
- `ApplyDepthOfField`
- `ApplyEnhancement`
- `ApplyFXAA`
- `ApplyGlow`
- `ApplySSAO`
- `BeginFrame`
- `BlurFrameBuffer`
- `BlurFrameBuffer2`
- `CalcTextRect` `C`
- `Clear`
- `ClearClippingPlane`
- `ClearClippingRect`
- `ClearRenderSurface`
- `CreateDynamicBuffers`
- `CreateTemporaryTextureSurface`
- `CreateTextureSurface`
- `CreateTextureSurface`
- `DestroyDynamicBuffers`
- `DestroySurface`
- `DrawDynamicRect`
- `DrawDynamicRect`
- `EnableWireframe`
- `EndFrame`
- `GammaCorrect`
- `GetClippingRect`
- `GetColorTarget` `C`
- `GetDeferredRendering` `C`
- `GetDepthTarget` `C`
- `GetDynamicIndexBuffer`
- `GetDynamicVertexBuffer`
- `GetHeight` `C`
- `GetPrimarySurface` `C`
- `GetQuadIndexBuffer`
- `GetRenderDevice`
- `GetSceneRenderer` `C`
- `GetViewport` `C`
- `GetWidth` `C`
- `GraphicsCanvas`
- `HotBlurFrameBuffer`
- `MaskFrameBuffer`
- `PostDeviceReset`
- `PreDeviceReset`
- `PresentSurface`
- `RecreateSceneRenderTargets`
- `RenderCircle`
- `RenderColoredText2d`
- `RenderColoredText2d`
- `RenderColoredText2d`
- `RenderFrustum`
- `RenderFrustum3`
- `RenderHorizontalGradient`
- `RenderLine`
- `RenderRadialWipeEffect`
- `RenderRect`
- `RenderRect`
- `RenderShadedRect`
- `RenderStyledRect`
- `RenderStyledRect`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2d`
- `RenderText2dBox`
- `RenderText2dBox`
- `RenderText2dBox`
- `RenderText2dParagraph`
- `RenderText3d`
- `RenderText3d`
- `RenderTriFan`
- `RenderVerticalGradient`
- `RenderWireframeRect`
- `SetClearColor`
- `SetClippingPlane`
- `SetClippingRect`
- `SetDefaultState`
- `SetDeferredRendering`
- `SetDepthTarget`
- `SetMultithreaded`
- `SetPrimarySurface`
- `SetRenderTarget`
- `SetResolution`
- `SetResolutionAdjustment`
- `SetTargetSurface`
- `SetViewport`
- `Supports16bitColorWithDepth` `C`
- `SupportsDepthRenderTargets` `C`
- `~GraphicsCanvas` `V`

### `GraphicsCursor` (Engine.dll, 6)

- `Destroy` `V`
- `GetTextureData` `C`
- `GetTextureSize` `C`
- `GraphicsCursor`
- `Initialize` `V`
- `~GraphicsCursor` `V`

### `GraphicsDeferredRenderer` (Engine.dll, 17)

- `ConstructRenderPass` `VC`
- `CreateRenderTargets` `V`
- `DestroyRenderTargets` `V`
- `ExecutePrepareScene` `V`
- `GetDepthBuffer` `VC`
- `GraphicsDeferredRenderer`
- `PerformTonemapping` `V`
- `RenderDebugSurface`
- `RenderDeferredAmbientLight`
- `RenderDeferredDirectionalLight`
- `RenderDeferredPointLight`
- `RenderFinalScene` `V`
- `RenderFog`
- `RenderSceneGlow`
- `RenderStencilBuffer`
- ``vftable'`
- `~GraphicsDeferredRenderer` `V`

### `GraphicsEngine` (Engine.dll, 112)

- `AlignPositionToScreen` `C`
- `AreDecalsEnabled` `C`
- `AreEffectsEnabled` `C`
- `AreGridTilesEnabled` `C`
- `AreMeshesEnabled` `C`
- `ArePostEffectsEnabled` `C`
- `AreReflectionsEnabled` `C`
- `AreShadowsEnabled` `C`
- `AreStatsEnabled` `C`
- `BeginPerfEvent` `C`
- `CreateDevice`
- `CreateDevice`
- `EnableBloom`
- `EnableCustomUIScaling`
- `EnableDecals`
- `EnableEffects`
- `EnableFPS`
- `EnableFog`
- `EnableGrass`
- `EnableGridTiles`
- `EnableLighting`
- `EnableMeshes`
- `EnableReflections`
- `EnableRegionList`
- `EnableRendering`
- `EnableShadows`
- `EnableShowBoundingBoxes`
- `EnableShowLights`
- `EnableStats`
- `EnableTerrain`
- `EnableTransparency`
- `EnableWater`
- `EnableWireframe`
- `EndPerfEvent` `C`
- `EvictOldResources`
- `ForceDeviceReset`
- `GetBasicShader` `C`
- `GetBasicVertexDeclaration` `C`
- `GetCanvas`
- `GetDefaultTexture` `C`
- `GetEmptyTexture` `C`
- `GetGenerationId` `C`
- `GetGenerationName` `C`
- `GetHdrLightingFactor` `C`
- `GetHeight` `C`
- `GetIsFogEnabled` `C`
- `GetMaxNumBones` `C`
- `GetOptimizationTest` `C`
- `GetOverlayShader` `C`
- `GetPixelShaderVersion` `C`
- `GetRenderDevice`
- `GetShadowDebugging`
- `GetStats`
- `GetSupportedDisplayModes`
- `GetTerrainQuality` `C`
- `GetTextureReduction` `C`
- `GetTextureReduction` `C`
- `GetUIScaleFactor` `C`
- `GetUIScaledRect` `C`
- `GetWidth` `C`
- `GraphicsEngine`
- `IncrementLightCount`
- `IncrementReflectionCount`
- `Initialize`
- `IsCustomUIScalingEnabled` `C`
- `IsFPSEnabled` `C`
- `IsFullscreen`
- `IsGrassEnabled` `C`
- `IsLightingEnabled` `C`
- `IsRegionListEnabled` `C`
- `IsRenderingEnabled` `C`
- `IsShowBoundingBoxesEnabled` `C`
- `IsShowLightsEnabled` `C`
- `IsTerrainEnabled` `C`
- `IsTransparencyEnabled` `C`
- `IsUIScalingEnabled` `C`
- `IsWaterEnabled` `C`
- `ListLoadedResources`
- `LoadAnimation`
- `LoadCursorA`
- `LoadFont`
- `LoadMesh`
- `LoadShader2`
- `LoadTexture`
- `PostDeviceReset`
- `PreDeviceReset`
- `PresentSurface`
- `SaveRenderSurface` `C`
- `SaveScreenShot` `C`
- `SetDirectoryTextureReduction`
- `SetDisplay`
- `SetGeneration`
- `SetGeneration`
- `SetHdrLightingFactor`
- `SetOptimizationTest`
- `SetOptionsToDefaults`
- `SetResolution`
- `SetShadowDebugging`
- `SetTextureReduction`
- `UnloadAllResources`
- `UnloadAnimation`
- `UnloadCursor`
- `UnloadFont`
- `UnloadMesh`
- `UnloadShader2`
- `UnloadTexture`
- `UnloadUnreferencedResources`
- `Update`
- `UpdateFromOptions`
- `UseLowQualityLighting` `C`
- `WriteResourceLog`
- `~GraphicsEngine` `V`

### `GraphicsEngineSettings` (Engine.dll, 6)

- `GraphicsEngineSettings`
- `GraphicsEngineSettings`
- `GraphicsEngineSettings`
- `operator=`
- `operator=`
- `~GraphicsEngineSettings`

### `GraphicsFont2` (Engine.dll, 14)

- `Destroy` `V`
- `GetKerningAmount` `C`
- `GetLineHeight` `C`
- `GetMetric` `C`
- `GetShader` `C`
- `GetStyle` `C`
- `GetSystemMemoryUsage` `VC`
- `GetTexture` `C`
- `GetVideoMemoryUsage` `VC`
- `GraphicsFont2`
- `Initialize` `V`
- `ProcessCharacter` `C`
- `ResetDynamicTexture` `C`
- `~GraphicsFont2` `V`

### `GraphicsLight` (Engine.dll, 29)

- `GetCastsShadows` `C`
- `GetCastsSpecular` `C`
- `GetColor` `C`
- `GetGroup` `C`
- `GetId` `C`
- `GetLinearColor` `C`
- `GetObjectToWorldCoords` `C`
- `GetRadius` `C`
- `GetShadowIntensity` `C`
- `GetType` `C`
- `GraphicsLight`
- `GraphicsLight`
- `GraphicsLight`
- `HasTargetRenderable` `C`
- `IsTargeted` `C`
- `SetCastsShadows`
- `SetCastsSpecular`
- `SetColor`
- `SetGroup`
- `SetId`
- `SetObjectToWorldCoords`
- `SetRadius`
- `SetShadowIntensity`
- `SetShadowPriority`
- `SetTargets`
- `SetType`
- `operator=`
- `operator=`
- `~GraphicsLight`

### `GraphicsMTRenderer` (Engine.dll, 123)

- `AddReflectionPlane`
- `AddRegionElementsToScene`
- `AddRegionToScene`
- `AddSceneLight`
- `BuildReflections` `V`
- `BuildScene`
- `BuildShadowDirectional`
- `BuildShadowPoint`
- `BuildShadows`
- `CalculateShadowBounds` `C`
- `ClearPendingJobs`
- `CollectPassesForAllLights` `C`
- `CollectPassesForLight` `C`
- `CollectPassesForSphere` `C`
- `CollectPassesForStyle` `C`
- `ConstructRenderPass` `VC`
- `CreateBuildReflectionJob`
- `CreateBuildSceneJob`
- `CreateBuildShadowJob`
- `CreatePrepareDepthJob`
- `CreatePrepareReflectionJob`
- `CreatePrepareSceneJob`
- `CreatePrepareShadowJob`
- `CreatePrepareTransparencyJob`
- `CreateReflectionGroup`
- `CreateRenderTargets` `V`
- `CreateScene`
- `DestroyBuildReflectionJob`
- `DestroyBuildSceneJob`
- `DestroyBuildShadowJob`
- `DestroyPrepareDepthJob`
- `DestroyPrepareReflectionJob`
- `DestroyPrepareSceneJob`
- `DestroyPrepareShadowJob`
- `DestroyPrepareTransparencyJob`
- `DestroyReflectionGroup`
- `DestroyRenderTargets` `V`
- `DestroyScene`
- `DrawLightBounds` `C`
- `ExecuteBuildReflection`
- `ExecuteBuildScene`
- `ExecuteBuildShadow`
- `ExecuteJob`
- `ExecutePrepareDepth`
- `ExecutePrepareReflection`
- `ExecutePrepareScene` `V`
- `ExecutePrepareShadow`
- `ExecutePrepareTransparency`
- `GetCameraTarget` `C`
- `GetCustomScene`
- `GetDepthBuffer` `VC`
- `GetDepthTexture` `C`
- `GetLightStyle` `C`
- `GetRenderMode` `C`
- `GetSphereScissorRect` `C`
- `GraphicsMTRenderer`
- `LightSpacePerspectiveShadowMapMatrix` `C`
- `PerformTonemapping` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `ProcessJobs`
- `QueueJob`
- `RecreateRenderTargets`
- `Render`
- `RenderBackground` `C`
- `RenderCamera`
- `RenderCustomScene`
- `RenderDebug`
- `RenderDepth`
- `RenderFinalScene` `V`
- `RenderPostEffects`
- `RenderPostObjects`
- `RenderReflection`
- `RenderScene`
- `RenderScenePasses` `C`
- `RenderShadow`
- `RenderShadowDirectional`
- `RenderShadowPoint`
- `RenderTransparentFeatures`
- `Reset`
- `SetAmbientOcclusionEnabled`
- `SetBackgroundMode`
- `SetBackgroundTexture`
- `SetCameraTarget`
- `SetClearFlags`
- `SetDepthFogParams`
- `SetDepthOfFieldEnabled`
- `SetDepthPassEnabled`
- `SetFXAAEnabled`
- `SetFogColor`
- `SetFogEnabled`
- `SetGroundAmbientColor`
- `SetHDRLighting`
- `SetHeightFogParams`
- `SetLightRigsEnabled`
- `SetLightingEnabled`
- `SetMultithreaded`
- `SetMultithreadingAllowed`
- `SetPostEffectsEnabled`
- `SetReflectionsEnabled`
- `SetRenderMode`
- `SetShaderParamsForScene` `C`
- `SetShaderParamsForShadow` `C`
- `SetShadowSoftness`
- `SetShadowsEnabled`
- `SetShowLights`
- `SetSkyAmbientColor`
- `SetTargetGlowEnabled`
- `SetWireframeEnabled`
- `SetupLight`
- `SetupLights`
- `TrapezoidalShadowMapMatrix` `C`
- `UniformShadowMapMatrix` `C`
- `UpdateFrameBufferCopy`
- `UpdateReflectionPlaneBounds`
- `WaitForJobs`
- ``vftable'`
- `kDirectionalShadowMatrix` `S`
- `kMaxLargePointShadowTargets` `S`
- `kMaxSmallPointShadowTargets` `S`
- `kNullSphere` `S`
- `kShadowCasterBoundMaxVertices` `S`
- `~GraphicsMTRenderer` `V`

### `GraphicsMesh` (Engine.dll, 65)

- `Destroy` `V`
- `GeometryBusStop` `VC`
- `GeometryBusStop` `VC`
- `GetAbsCoords` `C`
- `GetAbsCoords` `C`
- `GetAbsCoords` `C`
- `GetAbsCoords` `C`
- `GetAllAttachedPoints` `C`
- `GetAttachPoint` `C`
- `GetAttachedCoords` `C`
- `GetBone` `C`
- `GetBoneIndex` `C`
- `GetBoundingBox` `C`
- `GetContour` `C`
- `GetCreationData` `C`
- `GetFaceSet` `C`
- `GetFaces` `VC`
- `GetHitBox` `C`
- `GetHitBox` `C`
- `GetHitBox` `C`
- `GetHitBoxIndex` `C`
- `GetIndexBuffer` `C`
- `GetIsReadyToUse` `C`
- `GetJointDescription`
- `GetMIFData` `C`
- `GetMaterial` `C`
- `GetMeshRenderInfo` `C`
- `GetNormal` `VC`
- `GetNumBones` `C`
- `GetNumContours` `C`
- `GetNumCreationData` `C`
- `GetNumFaceSets` `C`
- `GetNumFaces` `C`
- `GetNumHitBoxes` `C`
- `GetNumJointDescriptions` `C`
- `GetNumMaterials` `C`
- `GetNumPortals` `C`
- `GetNumRigidBodyDescriptions` `C`
- `GetNumSections` `C`
- `GetNumSkeletonEmitterBones` `C`
- `GetNumVertices` `C`
- `GetOBBox` `VC`
- `GetPhysicsMesh` `C`
- `GetPortal` `C`
- `GetRigidBodyDescription` `C`
- `GetRootMotionBone` `C`
- `GetSection` `C`
- `GetSkeletonEmitterBone` `C`
- `GetSystemMemoryUsage` `VC`
- `GetVertexBuffer` `C`
- `GetVertexDeclaration` `C`
- `GetVerts` `VC`
- `GetVideoMemoryUsage` `VC`
- `GraphicsMesh`
- `HasRigidBodyData` `C`
- `HasVertexColors` `C`
- `Initialize` `V`
- `InitializeDefault` `V`
- `LogInfo` `VC`
- `PreLoadDependentResources` `V`
- `RenderBlendedFaces` `C`
- `RenderPaintableFaces` `C`
- `SetShaderParameters` `C`
- `UsesShaderOfType` `C`
- `~GraphicsMesh` `V`

### `GraphicsMeshInstance` (Engine.dll, 128)

- `BeginDissolve`
- `BeginFade`
- `BeginUnDissolve`
- `BeginUnFade`
- `CheckLOS` `C`
- `ClearBaseTextures`
- `ClearBumpTextures`
- `ClearGlowTextures`
- `ClearSpecTextures`
- `DisableDbrShader`
- `GetAllAttachedPoints`
- `GetAnimChannel`
- `GetBaseTexture`
- `GetBaseTexture`
- `GetBaseTextureName` `C`
- `GetBaseTextureName` `C`
- `GetBaseTexturesSize` `C`
- `GetBumpTexture`
- `GetBumpTexture`
- `GetBumpTextureName` `C`
- `GetBumpTextureName` `C`
- `GetBumpTexturesSize` `C`
- `GetCastsShadows` `VC`
- `GetDbrShaderName` `C`
- `GetDetailTexture`
- `GetDetailTexture`
- `GetDetailTextureName` `C`
- `GetDetailTextureName` `C`
- `GetDissolveParam` `C`
- `GetGlobalBlendAlphaMultiplier` `C`
- `GetGlowTexture`
- `GetGlowTexture`
- `GetGlowTextureName` `C`
- `GetGlowTextureName` `C`
- `GetGlowTexturesSize` `C`
- `GetIntersection` `C`
- `GetIsDissolved` `C`
- `GetIsDissolving` `C`
- `GetIsFaded` `C`
- `GetIsPlayingAnimation` `C`
- `GetIsUnDissolving` `C`
- `GetLightGroup` `VC`
- `GetMesh` `C`
- `GetNormal` `VC`
- `GetNumAnimChannels` `C`
- `GetNumRenderPasses` `VC`
- `GetNumSections` `C`
- `GetObjectSpaceBoundingBox` `C`
- `GetObjectToRegionCoords` `C`
- `GetOpacity` `VC`
- `GetPassFaces` `VC`
- `GetRegionSpaceABBox` `C`
- `GetRegionSpaceOBBox` `C`
- `GetRenderPassBoundingBox` `VC`
- `GetScale` `C`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetShadowBias` `C`
- `GetSkeletalPose`
- `GetSpecTexture`
- `GetSpecTexture`
- `GetSpecTextureName` `C`
- `GetSpecTextureName` `C`
- `GetSpecTexturesSize` `C`
- `GetTexture` `VC`
- `GraphicsMeshInstance`
- `GraphicsMeshInstance`
- `InstallGMIEffect`
- `IsAnimationValid`
- `IsTransparent` `C`
- `IsValid` `C`
- `LogInfo` `VC`
- `PreLoad`
- `PrepareForRendering`
- `ReadReplicationData`
- `RemoveGMIListener`
- `RenderBoundingBox` `C`
- `RenderHitBoxes` `C`
- `RenderPass` `VC`
- `RestoreState`
- `SaveState`
- `SetBaseTexture`
- `SetBaseTexture`
- `SetBoneTransparancies`
- `SetBumpTexture`
- `SetBumpTexture`
- `SetCastsShadows`
- `SetDbrOverrideShader`
- `SetDbrShader`
- `SetDetailTexture`
- `SetDetailTexture`
- `SetDiffuseColor`
- `SetGlobalBlendAlphaMultiplier`
- `SetGlowTexture`
- `SetGlowTexture`
- `SetHighlight`
- `SetIgnoreLightShadows` `V`
- `SetLightGroup`
- `SetMaxGlobalBlendAlphaMultiplier`
- `SetMesh`
- `SetMesh`
- `SetObjectToRegionCoords`
- `SetOutlineColor`
- `SetOutlineThickness`
- `SetOutlined`
- `SetOverrideShader`
- `SetPoseFromObjectSpace`
- `SetPoseFromObjectSpace`
- `SetScale`
- `SetSectionOpacity`
- `SetShaderParameters` `VC`
- `SetShadowBias`
- `SetSkeletalPose`
- `SetSpecTexture`
- `SetSpecTexture`
- `SetTransparency`
- `SetTransparent`
- `SetTransparentDbrShader`
- `SetUnloadedBoundingBoxExtents`
- `UnloadOverrideShader`
- `Update`
- `UpdateBoundingBox`
- `UpdatePose`
- `UpdateRegionSpaceBoundingBox`
- `UpdateSectionOpacities`
- `WriteReplicationData`
- `operator=`
- `~GraphicsMeshInstance` `V`

### `GraphicsPrimitiveDrawer` (Engine.dll, 24)

- `Begin`
- `Enable2DMode`
- `End`
- `Flush`
- `GetCamera` `C`
- `GetCameraRegion` `C`
- `GetCanvas`
- `GetViewport`
- `GraphicsPrimitiveDrawer`
- `Is2DMode`
- `SetCamera`
- `SetColor`
- `SetRegion`
- `SetShader`
- `SetShader`
- `SetTexCoord`
- `SetTexture`
- `SetTexture0`
- `SetTexture1`
- `SetVertex`
- `SetVertex`
- `SetVertexBuilder`
- `SetWorldToScreenMatrix`
- `~GraphicsPrimitiveDrawer` `V`

### `GraphicsRenderable` (Engine.dll, 9)

- `GetCastsShadows` `VC`
- `GetLightGroup` `VC`
- `GetOpacity` `VC`
- `GraphicsRenderable`
- `GraphicsRenderable`
- `GraphicsRenderable`
- ``vftable'`
- `operator=`
- `operator=`

### `GraphicsScene` (Engine.dll, 40)

- `AddLight`
- `AddLightGroup` `C`
- `AddRegion`
- `AddRenderable`
- `CalculateSceneBounds` `C`
- `GetCamera` `C`
- `GetCameraRegion` `C`
- `GetCameraToScreenMatrix` `C`
- `GetCameraToWorldMatrix` `C`
- `GetFrustum` `C`
- `GetLights`
- `GetLights` `C`
- `GetRegionToSceneCoords` `C`
- `GetRegionToSceneCoords` `C`
- `GetRenderablePasses`
- `GetRenderables`
- `GetRenderables` `C`
- `GetRenderer` `C`
- `GetSceneType` `C`
- `GetScreenToCameraMatrix` `C`
- `GetState` `C`
- `GetViewport` `C`
- `GetWorldToCameraMatrix` `C`
- `GetWorldToScreenMatrix` `VC`
- `GraphicsScene`
- `GraphicsScene`
- `IsRenderFlagSet` `C`
- `Release`
- `SetFrustum`
- `SetReady`
- `SetRegionToSceneCoords`
- `SetRenderFlags`
- `SetRendered`
- `SetRendering`
- `SetSceneType`
- `SetViewer`
- `SetViewport`
- `SetWorldToScreenMatrix`
- ``vftable'`
- `~GraphicsScene`

### `GraphicsShader2` (Engine.dll, 30)

- `Begin` `C`
- `Begin` `C`
- `Destroy` `V`
- `End` `C`
- `FindStyleByName` `C`
- `GetIsReadyToUse` `C`
- `GetSortOrder` `C`
- `GetStyleName` `C`
- `GetSystemMemoryUsage` `VC`
- `GetVideoMemoryUsage` `VC`
- `GraphicsShader2`
- `HasParameter` `C`
- `Initialize` `V`
- `PreLoadDependentResources` `V`
- `Render` `C`
- `Render` `C`
- `SetFloat` `C`
- `SetFloat2` `C`
- `SetFloat2` `C`
- `SetFloat3` `C`
- `SetFloat3` `C`
- `SetFloat3` `C`
- `SetFloat4` `C`
- `SetFloat4x3` `C`
- `SetFloat4x3` `C`
- `SetFloat4x4` `C`
- `SetFloat4x4` `C`
- `SetInt` `C`
- `SetTexture` `C`
- `~GraphicsShader2` `V`

### `GraphicsTexture` (Engine.dll, 17)

- `Destroy` `V`
- `GetDataLength` `V`
- `GetDebugInfo` `VC`
- `GetFrameRate` `C`
- `GetHeight` `C`
- `GetIsReadyToUse` `C`
- `GetNumFrames` `C`
- `GetRect` `C`
- `GetSystemMemoryUsage` `VC`
- `GetTexture` `C`
- `GetTexture` `C`
- `GetVideoMemoryUsage` `VC`
- `GetWidth` `C`
- `GraphicsTexture`
- `Initialize` `V`
- `InitializeDefault` `V`
- `~GraphicsTexture` `V`

### `GrassObject` (Engine.dll, 23)

- `AddDisturbance` `V`
- `AddDisturbance` `V`
- `AddWind` `V`
- `CleanOffsetField`
- `CreateGrass` `V`
- `CreateGrassBuffers` `V`
- `DestroyGrass` `V`
- `DestroyGrass` `V`
- `GetGrassBlockByIndex` `C`
- `GrassObject`
- `GrassObject`
- `SetupGrassGeometry` `V`
- `UpdateOffsetField` `V`
- ``vftable'`
- `kGrassDisabled` `S`
- `kGrassHighQualitySpacing` `S`
- `kGrassLowQualityMinHeight` `S`
- `kGrassLowQualityMultiplier` `S`
- `kGrassLowQualitySpacing` `S`
- `kGrassMovementPeriod` `S`
- `kGrassNotReady` `S`
- `operator=`
- `~GrassObject` `V`

### `GridBase` (Engine.dll, 32)

- `AddToScene` `V`
- `GeometryBusStop` `V`
- `GeometryBusStop` `V`
- `GetBoundingBox` `VC`
- `GetCellSize` `VC`
- `GetDepth` `VC`
- `GetDepthInCells` `VC`
- `GetHeight` `VC`
- `GetHeightInCells` `VC`
- `GetIntersection` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRegion` `VC`
- `GetStaticClassInfo` `S`
- `GetWidth` `VC`
- `GetWidthInCells` `VC`
- `GridBase`
- `GridBase`
- `GridBase`
- `InvalidateBoundingBox` `V`
- `Load` `V`
- `PostLoad` `V`
- `PreLoad` `V`
- `PreLoadFrustums` `V`
- `RTTI_new` `S`
- `Resize` `V`
- `Save` `VC`
- `Update` `V`
- `UpdateBoundingBox` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~GridBase` `V`

### `GridRegion` (Engine.dll, 61)

- `AddToScene` `V`
- `BuildCellSpace`
- `CellContainsFeature` `C`
- `ChangeCellVariation`
- `ClearTransparency`
- `DestroyCell`
- `DestroyCells`
- `GeometryBusStop`
- `GeometryBusStop` `V`
- `GetCell`
- `GetCell` `C`
- `GetCellCenter` `C`
- `GetCellCoords` `C`
- `GetCellFeature` `C`
- `GetCellFloor` `C`
- `GetCellMeshesInBox` `C`
- `GetCellSize` `VC`
- `GetCellVariation` `C`
- `GetDepth` `VC`
- `GetDepthInCells` `VC`
- `GetGroundHeight` `C`
- `GetHeight` `VC`
- `GetHeightInCells` `VC`
- `GetHighestVisibleFloor` `C`
- `GetIntersection` `VC`
- `GetLatticeValue` `C`
- `GetMeshesInFrustum` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetSystem` `C`
- `GetSystemFileName` `C`
- `GetWidth` `VC`
- `GetWidthInCells` `VC`
- `GridRegion`
- `GridRegion`
- `IsRandom` `VC`
- `Load` `V`
- `PostLoad` `V`
- `PreLoadFrustums` `V`
- `RTTI_new` `S`
- `ReadLatticeChunk`
- `RenderLattice` `C`
- `RenderLattice` `C`
- `Resize` `V`
- `Rotate`
- `Save` `VC`
- `SetCellFeature`
- `SetCellFloor`
- `SetCellVariation`
- `SetHighestVisibleFloor`
- `SetLatticeValue`
- `SetSystem`
- `SpreadTransparency`
- `Update` `V`
- `UpdateCell`
- `UpdateTransparency`
- `UpdateTransparency`
- ``vftable'`
- `cellsPerChunk` `S`
- `classInfo` `S`
- `~GridRegion` `V`

### `GridRegionRandom` (Engine.dll, 10)

- `GridRegionRandom`
- `IsRandom` `VC`
- `IsRoomConnected`
- `Load` `V`
- `NUM_DOOR_TYPES` `S`
- `NUM_ROOM_TYPES` `S`
- `Randomize`
- `Save` `VC`
- ``vftable'`
- `~GridRegionRandom` `V`

### `GridSystem` (Engine.dll, 24)

- `GetBumpTexture` `C`
- `GetCellSpacing` `C`
- `GetDiffuseTexture` `C`
- `GetFeatureMesh` `C`
- `GetFeatureName` `C`
- `GetFeatureTexturesOverriden` `C`
- `GetMeshForCell` `C`
- `GetMinWallOpacity` `C`
- `GetNumFeatures` `C`
- `GetNumVariationsForCell` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWallPiece` `C`
- `GridSystem`
- `GridSystem`
- `Load` `V`
- `LoadWallPiece`
- `PreLoad`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `defaultCellSpacing` `S`
- `operator=`
- `~GridSystem` `V`

### `Guard` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Guard`
- `Load` `V`
- `RTTI_new` `S`
- `WriteSimulationInformation` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Guard` `V`

### `HallOfFameCamera` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HallOfFameCamera`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `ShouldServerSpawn` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~HallOfFameCamera` `V`

### `HallOfFameStand` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HallOfFameStand`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `ShouldServerSpawn` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~HallOfFameStand` `V`

### `HeartbeatPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `HeartbeatPacket`
- `HeartbeatPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~HeartbeatPacket` `V`

### `HeartbeatResponsePacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `HeartbeatResponsePacket`
- `HeartbeatResponsePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~HeartbeatResponsePacket` `V`

### `Hireling` (Game.dll, 13)

- `Employ`
- `Fire`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Hireling`
- `IsHireable` `C`
- `OnAddToWorld` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Hireling` `V`

### `HookPack` (Engine.dll, 13)

- `ExecuteEventHook` `C`
- `ExecuteQueryHook` `C`
- `HookPack`
- `HookPack`
- `Initialize`
- `IsHooked` `C`
- `LoadEventHook`
- `LoadFromFile` `V`
- `LoadFromTable` `V`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~HookPack` `V`

### `HotSlotOption` (Game.dll, 35)

- `DeepCopy`
- `GetActiveCharges` `VC`
- `GetBitmapMask` `V`
- `GetBitmapNameDown` `V`
- `GetBitmapNameUp` `V`
- `GetCooldownCompletion` `V`
- `GetCooldownRemaining` `V`
- `GetDisplayName` `VC`
- `GetMaskBottomOffset` `V`
- `GetMaskTopOffset` `V`
- `GetMaxCharges` `VC`
- `GetNumberAvailable` `V`
- `GetRolloverText` `VC`
- `GetRoundBitmapNameDown` `V`
- `GetRoundBitmapNameUp` `V`
- `GetSkillId` `VC`
- `GetStatus` `V`
- `GetType` `C`
- `HotSlotOption`
- `HotSlotOption`
- `HotSlotOption`
- `IsActiveDelay` `C`
- `IsRepeatable` `VC`
- `PlayCooldownVox` `V`
- `ReadProperties` `V`
- `SetActiveDelay`
- `SetPlayer`
- `StreamProperties` `V`
- `Update`
- `Validate` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `operator=`
- `operator==` `VC`
- `~HotSlotOption` `V`

### `HotSlotOptionEvade` (Game.dll, 12)

- `Activate` `V`
- `HotSlotOptionEvade`
- `HotSlotOptionEvade`
- `HotSlotOptionEvade`
- `HotSlotOptionEvade`
- `ReadProperties` `V`
- `StreamProperties` `V`
- `Validate` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `operator=`
- `~HotSlotOptionEvade` `V`

### `HotSlotOptionPotion` (Game.dll, 25)

- `Activate` `V`
- `GetBitmapNameDown` `V`
- `GetBitmapNameUp` `V`
- `GetCooldownCompletion` `V`
- `GetCooldownRemaining` `V`
- `GetDisplayName` `VC`
- `GetNumberAvailable` `V`
- `GetRolloverText` `VC`
- `GetStatus` `V`
- `HealthBitmapDownName` `S`
- `HealthBitmapUpName` `S`
- `HealthTextDisplayName` `S`
- `HotSlotOptionPotion`
- `HotSlotOptionPotion`
- `HotSlotOptionPotion`
- `HotSlotOptionPotion`
- `ManaBitmapDownName` `S`
- `ManaBitmapUpName` `S`
- `ManaTextDisplayName` `S`
- `SetDefaultPotionData` `S`
- `Validate` `V`
- ``vftable'`
- `operator=`
- `operator==` `VC`
- `~HotSlotOptionPotion` `V`

### `HotSlotOptionPotionSkill` (Game.dll, 18)

- `Activate` `V`
- `GetBitmapMask` `V`
- `GetBitmapNameDown` `V`
- `GetBitmapNameUp` `V`
- `GetMaskBottomOffset` `V`
- `GetMaskTopOffset` `V`
- `HotSlotOptionPotionSkill`
- `HotSlotOptionPotionSkill`
- `HotSlotOptionPotionSkill`
- `HotSlotOptionPotionSkill`
- `PlayCooldownVox` `V`
- `ReadProperties` `V`
- `StreamProperties` `V`
- `Validate` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `operator=`
- `~HotSlotOptionPotionSkill` `V`

### `HotSlotOptionScroll` (Game.dll, 22)

- `Activate` `V`
- `GetBitmapNameDown` `V`
- `GetBitmapNameUp` `V`
- `GetCooldownCompletion` `V`
- `GetCooldownRemaining` `V`
- `GetDisplayName` `VC`
- `GetNumberAvailable` `V`
- `GetRolloverText` `VC`
- `GetStatus` `V`
- `HotSlotOptionScroll`
- `HotSlotOptionScroll`
- `HotSlotOptionScroll`
- `HotSlotOptionScroll`
- `ReadProperties` `V`
- `SetInfo`
- `StreamProperties` `V`
- `Validate` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `operator=`
- `operator==` `VC`
- `~HotSlotOptionScroll` `V`

### `HotSlotOptionSkill` (Game.dll, 34)

- `Activate` `V`
- `GetActiveCharges` `VC`
- `GetBitmapNameDown` `V`
- `GetBitmapNameUp` `V`
- `GetCooldownCompletion` `V`
- `GetCooldownRemaining` `V`
- `GetDisplayName` `VC`
- `GetMaxCharges` `VC`
- `GetNumberAvailable` `V`
- `GetRolloverText` `VC`
- `GetRoundBitmapNameDown` `V`
- `GetRoundBitmapNameUp` `V`
- `GetSkillId` `VC`
- `GetSkillRecord` `C`
- `GetStatus` `V`
- `HotSlotOptionSkill`
- `HotSlotOptionSkill`
- `HotSlotOptionSkill`
- `HotSlotOptionSkill`
- `IsRepeatable` `VC`
- `PlayCooldownVox` `V`
- `ReadProperties` `V`
- `ResolveSkillId` `C`
- `SetSimpleText`
- `SetSkillId`
- `StreamProperties` `V`
- `Validate` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `kAlternateEquipmentFlag` `S`
- `kAlternateEquipmentMask` `S`
- `operator=`
- `operator==` `VC`
- `~HotSlotOptionSkill` `V`

### `HwndWindow` (Engine.dll, 31)

- `Activate` `V`
- `Center` `V`
- `Close` `V`
- `DisableSetCursor` `V`
- `GetClientHeight` `VC`
- `GetClientWidth` `VC`
- `GetHeight` `VC`
- `GetMonitorRect` `V`
- `GetStyle` `VC`
- `GetSystemWindow` `VC`
- `GetWidth` `VC`
- `GetX` `VC`
- `GetY` `VC`
- `HwndWindow`
- `HwndWindow`
- `Initialize` `V`
- `IsActive` `V`
- `Maximize` `V`
- `Minimize` `V`
- `ProcessMessages` `V`
- `RegisterEventHandler` `V`
- `SetCaption` `V`
- `SetCursor` `V`
- `SetGammaRamp` `V`
- `SetSize` `V`
- `SetTopmost` `V`
- `Show` `V`
- `UnregisterEventHandler` `V`
- ``vftable'`
- `operator=`
- `~HwndWindow` `V`

### `IGPDVertexBuilder` (Engine.dll, 6)

- `IGPDVertexBuilder`
- `IGPDVertexBuilder`
- `IGPDVertexBuilder`
- ``vftable'`
- `operator=`
- `operator=`

### `IOAtomicRead` (Engine.dll, 1)

- `IOAtomicRead`

### `IOAtomicWrite` (Engine.dll, 2)

- `IOAtomicWrite`
- `Shutdown` `V`

### `IOStream` (Engine.dll, 1)

- `IOStream`

### `IOStreamRead` (Engine.dll, 31)

- `BeginBlock` `V`
- `EndBlock` `V`
- `GetFileBuffer` `C`
- `GetFileLength` `C`
- `HandlePropertyTag` `V`
- `IOStreamRead`
- `IOStreamRead`
- `IOStreamRead`
- `Initialize`
- `IsReading` `V`
- `IsValid` `VC`
- `Rewind` `V`
- `Shutdown` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- ``default constructor closure'`
- `~IOStreamRead` `V`

### `IOStreamWrite` (Engine.dll, 27)

- `BeginBlock` `V`
- `EndBlock` `V`
- `GetFileBuffer` `C`
- `GetFileLength` `C`
- `HandlePropertyTag` `V`
- `IOStreamWrite`
- `IsReading` `V`
- `IsValid` `VC`
- `Rewind` `V`
- `Shutdown` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `StreamPropertyEx` `V`
- `~IOStreamWrite` `V`

### `IPVNetworkAddress` (Engine.dll, 26)

- `Any` `S`
- `FromNetworkAddress` `S`
- `GetAddrIPV4` `C`
- `GetAddressAsSockAddr` `C`
- `GetAddressType` `C`
- `GetPort` `C`
- `GetSockAddrSize` `C`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `IPVNetworkAddress`
- `Invalidate`
- `IsIPAddressEqual` `C`
- `IsValid` `VC`
- `SetPort`
- `ToString` `VC`
- ``vftable'`
- `operator!=` `C`
- `operator=`
- `operator=`
- `operator==` `C`

### `IPhysics2` (Engine.dll, 13)

- `IPhysics2`
- `IPhysics2`
- `IPhysics2`
- `PhysicsCollision` `V`
- `PhysicsGetSurfaceType` `V`
- `PhysicsPost` `V`
- `PhysicsResponse` `V`
- `PhysicsSync` `V`
- `PhysicsTest` `V`
- `PhysicsUpdate` `V`
- ``vftable'`
- `operator=`
- `operator=`

### `ITTEvent` (Engine.dll, 2)

- `ITTEvent`
- `~ITTEvent`

### `IdleAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `IdleAction`
- `IdleAction`
- `ToString` `VC`
- ``vftable'`
- `~IdleAction` `V`

### `IdleActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `IdleActionPacket`
- `IdleActionPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~IdleActionPacket` `V`

### `Image` (Engine.dll, 19)

- `ChangeFormat`
- `Create`
- `Destroy`
- `FlipDiagonally`
- `FlipHorizontally`
- `FlipVertically`
- `GetBitsPerPixel` `C`
- `GetBuffer` `C`
- `GetFormat` `C`
- `GetHeight` `C`
- `GetPitch` `C`
- `GetWidth` `C`
- `Image`
- `Load`
- `Load`
- `Save`
- `WriteTGA` `C`
- `operator=`
- `~Image` `V`

### `ImageResource` (Engine.dll, 12)

- `Destroy` `V`
- `GetChannels` `C`
- `GetData` `C`
- `GetDataSize` `C`
- `GetHeight` `C`
- `GetIsReadyToUse` `C`
- `GetSystemMemoryUsage` `VC`
- `GetWidth` `C`
- `ImageResource`
- `Initialize` `V`
- `InitializeDefault` `V`
- `~ImageResource` `V`

### `ImmobilizeAction` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ImmobilizeAction`
- `ImmobilizeAction`
- `ToString` `VC`
- ``vftable'`
- `~ImmobilizeAction` `V`

### `ImmobilizePacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `ImmobilizePacket`
- `ImmobilizePacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~ImmobilizePacket` `V`

### `ImpassableData` (Engine.dll, 57)

- `AddBox`
- `AddEntity`
- `AddPathFace`
- `CleanForLoad`
- `Clear`
- `ClearCell`
- `DebugRenderBoxes`
- `DisableInvalidate`
- `EnableInvalidate`
- `GetCellMeshesInBox`
- `GetCollisionBuffer`
- `GetCollisionBufferLength`
- `GetHeight`
- `GetImpassable` `C`
- `GetImpassableBuffer`
- `GetInvisible` `C`
- `GetIsValid`
- `GetOverlappingBoxes`
- `GetOverlappingPathFaces`
- `GetPathBuffer`
- `GetPathBufferLength`
- `GetPhysicsFaceData`
- `GetTokBuffer`
- `GetTokBufferLength`
- `HasBeenLoadedOrPrimed` `C`
- `ImpassableData`
- `Invalidate`
- `Load`
- `Lock`
- `MirrorX`
- `MirrorXZ`
- `MirrorZ`
- `PolygonIsInsideBox`
- `PrimeForGrid`
- `PrimeForTerrain`
- `RecalculateRegionBox`
- `RemoveEntity`
- `RemoveIDBoxes`
- `RemoveIDFaces`
- `RenderPathMesh`
- `Save`
- `SetBoundingBox`
- `SetCollisionBuffer`
- `SetHeight`
- `SetImpassable`
- `SetInvisible`
- `SetOffset`
- `SetPathBuffer`
- `SetPhysicsFaceData`
- `SetPhysicsOffsetToWorldCoords`
- `SetRegion`
- `SetRegionHint`
- `SetTokBuffer`
- `ShiftBoxes`
- `Unlock`
- `Validate`
- `~ImpassableData`

### `IncBaseDexterityConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `IncBaseDexterityConfigCmdPacket`
- `IncBaseDexterityConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~IncBaseDexterityConfigCmdPacket` `V`

### `IncBaseIntelligenceConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `IncBaseIntelligenceConfigCmdPacket`
- `IncBaseIntelligenceConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~IncBaseIntelligenceConfigCmdPacket` `V`

### `IncBaseLifeConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `IncBaseLifeConfigCmdPacket`
- `IncBaseLifeConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~IncBaseLifeConfigCmdPacket` `V`

### `IncBaseManaConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `IncBaseManaConfigCmdPacket`
- `IncBaseManaConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~IncBaseManaConfigCmdPacket` `V`

### `IncBaseStrengthConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `IncBaseStrengthConfigCmdPacket`
- `IncBaseStrengthConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~IncBaseStrengthConfigCmdPacket` `V`

### `IncrementBaseDexterityConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `IncrementBaseDexterityConfigCmd`
- `IncrementBaseDexterityConfigCmd`
- ``vftable'`
- `operator=`
- `~IncrementBaseDexterityConfigCmd` `V`

### `IncrementBaseIntelligenceConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `IncrementBaseIntelligenceConfigCmd`
- `IncrementBaseIntelligenceConfigCmd`
- ``vftable'`
- `operator=`
- `~IncrementBaseIntelligenceConfigCmd` `V`

### `IncrementBaseLifeConfigCmd` (Game.dll, 8)

- `Execute` `V`
- `GetNetPacket` `V`
- `IncrementBaseLifeConfigCmd`
- `IncrementBaseLifeConfigCmd`
- `IncrementBaseLifeConfigCmd`
- ``vftable'`
- `operator=`
- `~IncrementBaseLifeConfigCmd` `V`

### `IncrementBaseManaConfigCmd` (Game.dll, 8)

- `Execute` `V`
- `GetNetPacket` `V`
- `IncrementBaseManaConfigCmd`
- `IncrementBaseManaConfigCmd`
- `IncrementBaseManaConfigCmd`
- ``vftable'`
- `operator=`
- `~IncrementBaseManaConfigCmd` `V`

### `IncrementBaseStrengthConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `IncrementBaseStrengthConfigCmd`
- `IncrementBaseStrengthConfigCmd`
- ``vftable'`
- `operator=`
- `~IncrementBaseStrengthConfigCmd` `V`

### `InputDevice` (Engine.dll, 11)

- `Create` `S`
- `Destroy` `S`
- `GetDefaultButtonName` `C`
- `GetDefaultShortButtonName` `C`
- `InputDevice`
- `InputDevice`
- ``vftable'`
- `createImpl` `S`
- `destroyImpl` `S`
- `operator=`
- `~InputDevice` `V`

### `InspectHelperPacket` (Game.dll, 36)

- `CopyInbound` `V`
- `FillInfo`
- `GetArtifactInfo` `C`
- `GetChestInfo` `C`
- `GetEquipAlternate` `C`
- `GetFeetInfo` `C`
- `GetFinger1Info` `C`
- `GetFinger2Info` `C`
- `GetGreatestDamageInflicted` `C`
- `GetGreatestMonsterLevel` `C`
- `GetGreatestMonsterName` `C`
- `GetHandsInfo` `C`
- `GetHeadInfo` `C`
- `GetLeftHandInfo` `C`
- `GetLegsInfo` `C`
- `GetMedalInfo` `C`
- `GetNeckInfo` `C`
- `GetNumberDeaths` `C`
- `GetNumberKilled` `C`
- `GetPacketDescription` `V`
- `GetPlayDays` `C`
- `GetPlayHours` `C`
- `GetPlayMinutes` `C`
- `GetPlayerId` `C`
- `GetRequesterId` `C`
- `GetRightHandInfo` `C`
- `GetShouldersInfo` `C`
- `GetWaistInfo` `C`
- `InspectHelperPacket`
- `InspectHelperPacket`
- `PrepareOutBuffer` `V`
- `SetRequesterId`
- `SyphonData`
- ``vftable'`
- `operator=`
- `~InspectHelperPacket` `V`

### `InspectRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `InspectRequestPacket`
- `InspectRequestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~InspectRequestPacket` `V`

### `InstanceGroup` (Engine.dll, 32)

- `AddChild`
- `AddEntity`
- `CalcCenter`
- `FillIdList`
- `FilterEntity`
- `GetCenter`
- `GetChildId` `C`
- `GetEntityData`
- `GetEntityData`
- `GetGroupID` `C`
- `GetID`
- `GetName` `C`
- `GetNumChildren` `C`
- `GetNumID`
- `GetType` `C`
- `GetTypeString` `C`
- `InstanceGroup`
- `InstanceGroup`
- `ListContains`
- `Load`
- `MoveEntityDown`
- `MoveEntityUp`
- `RemoveChild`
- `RemoveEntity`
- `RemoveEntity`
- `Render`
- `Save`
- `SetName`
- `SetType`
- `UpdateEntityData`
- `operator=`
- `~InstanceGroup`

### `InstanceGroupManager` (Engine.dll, 28)

- `AddGroupAffiliation`
- `CreateNewGroup`
- `DeleteGroup`
- `Destroy` `S`
- `DisavowAllKnowledge`
- `FilterEntity` `C`
- `Get` `S`
- `GetEntityData`
- `GetGroup`
- `GetGroup`
- `GetGroup`
- `GetGroupIndex`
- `GetGroupTypes`
- `GetGroupsOfType` `C`
- `GetGroupsThatContain` `C`
- `GetKeyIndex`
- `GetNumGroups` `C`
- `InstanceGroupManager`
- `IsGrouped` `C`
- `Load`
- `LockGroupList`
- `RemoveAllLinksTo`
- `Save` `C`
- `UnlockGroupList`
- `UpdateEntityData`
- `WorldGroupSelection` `C`
- `manager` `S`
- `~InstanceGroupManager`

### `InteractableHookPack` (Game.dll, 6)

- `InteractableHookPack`
- `InteractableHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~InteractableHookPack` `V`

### `InterfaceSkin` (Engine.dll, 7)

- `GetArea` `C`
- `InterfaceSkin`
- `InterfaceSkin`
- `LoadUIS`
- ``vftable'`
- `operator=`
- `~InterfaceSkin` `V`

### `InternalParam` (Engine.dll, 20)

- `GetAutoSpacing` `C`
- `GetEndPos` `C`
- `GetFloat` `VC`
- `GetGender` `VC`
- `GetIndex` `C`
- `GetStartPos` `C`
- `Init`
- `InternalParam`
- `InternalParam`
- `ReadInt` `S`
- `Set` `V`
- `Set` `V`
- `Set` `V`
- `SetEndPos`
- `SetIndex`
- `SetStartPos`
- `StringToGender` `C`
- ``vftable'`
- `operator=`
- `~InternalParam` `V`

### `Intersection` (Engine.dll, 3)

- `Intersection`
- `operator=`
- `operator=`

### `Inventory` (Game.dll, 19)

- `AddItemToInventory`
- `CalculateAllocatedMemory` `C`
- `DestroyAllObjects`
- `GetCompatible`
- `GetInventoryItems` `C`
- `GetItemCount` `C`
- `HasItemInInventory` `C`
- `Inventory`
- `Inventory`
- `IsItemInInventory` `C`
- `IsItemInInventory` `C`
- `OnItemAdd`
- `PopInventoryItem`
- `RemoveItemFromInventory`
- `SetGender`
- `SetOwner`
- ``vftable'`
- `operator=`
- `~Inventory` `V`

### `InventorySack` (Game.dll, 58)

- `AddItem`
- `AddItem`
- `AddItemAndReturnPoint`
- `AlignRect` `C`
- `ArrangeUnpositionedItems`
- `CompleteRelics`
- `ContainsItem` `C`
- `ContainsItem` `C`
- `DeleteAndCreateAllItemsFor`
- `DepositSackIntoReagents`
- `DestroyAllItems`
- `FindNextPosition` `C`
- `GetBorderColorIndex` `C`
- `GetBorderIndex` `C`
- `GetButtonName` `C`
- `GetCellHeight` `C`
- `GetCellWidth` `C`
- `GetConflicts` `C`
- `GetFirstItem`
- `GetGridHeight` `C`
- `GetGridWidth` `C`
- `GetInventory`
- `GetInventory` `C`
- `GetItemCount` `C`
- `GetItemPosition` `C`
- `GetItemUnderPoint` `C`
- `GetNotFullItem` `C`
- `GetNotFullRelic` `C`
- `GetRectUnderPoint` `C`
- `GetSymbolColorIndex` `C`
- `GetSymbolIndex` `C`
- `GiveAllItems`
- `GridToPixels` `C`
- `InventorySack`
- `InventorySack`
- `IsConflict` `C`
- `IsItemAddedWhileNotTheCurrentlySelectedInventoryTab`
- `IsSpaceForItem` `C`
- `LoadSackIntoTransmutes`
- `PixelsToGrid` `C`
- `RemoveAllItems`
- `RemoveItem`
- `ReplaceItem`
- `SearchHorizantal` `C`
- `SearchItems`
- `SearchVertical` `C`
- `SetBorder`
- `SetBorderColor`
- `SetButtonName`
- `SetDims`
- `SetItemAddedWhileNotTheCurrentlySelectedInventoryTab`
- `SetSymbol`
- `SetSymbolColor`
- `Sort`
- `TakeAllItems`
- ``vftable'`
- `operator=`
- `~InventorySack` `V`

### `InvitePartyConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `InvitePartyConfigCmdPacket`
- `InvitePartyConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~InvitePartyConfigCmdPacket` `V`

### `Item` (Game.dll, 181)

- `AddTransmute` `V`
- `AllowUse` `VC`
- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `AreRequirementsMet` `VC`
- `AttachItem` `V`
- `CanAutoPickup` `VC`
- `CanBePlacedInTransferStash` `VC`
- `CanReroll` `C`
- `CannotPickUp` `C`
- `CannotPickUpMultiple` `C`
- `CheckPlayerVisibility` `C`
- `ClearAltPickUpLocation`
- `CollisionCallback` `V`
- `ContributeRacialBonusDamage` `VC`
- `ContributeRacialBonusDefense` `VC`
- `CreateBeamEnd`
- `CreateBeamLoop`
- `CreateItem` `S`
- `CreatePrimaryCursorHandler` `V`
- `CreateSecondaryCursorHandler` `V`
- `DLCRequirementMet` `C`
- `DecrementStack` `V`
- `DetachItem` `V`
- `Dissolve`
- `DumpCostAttributes` `V`
- `GenerateRequirementText` `C`
- `GenerateSearchText`
- `GetAffixRerolls` `C`
- `GetAltPickUpLocation` `C`
- `GetAutoPickupRadius` `VC`
- `GetBaseItemCost` `C`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetCharAttributes` `VC`
- `GetCharacter` `C`
- `GetCollisionType` `VC`
- `GetConversionAttributes` `VC`
- `GetCraftingDisplay` `C`
- `GetDefenseAttributes` `VC`
- `GetDexterityRequirement` `VC`
- `GetDropClassification` `C`
- `GetFullItemDescription` `VC`
- `GetGMIEffect` `C`
- `GetGameDescription` `VC`
- `GetHintTag` `VC`
- `GetIntelligenceRequirement` `VC`
- `GetItemClassification` `VC`
- `GetItemCost` `VC`
- `GetItemHooks`
- `GetItemLevel` `C`
- `GetItemMarketType` `VC`
- `GetItemReplicaInfo` `VC`
- `GetItemTextTag` `C`
- `GetItemType` `VC`
- `GetItemTypeTag` `S`
- `GetItemVersion` `C`
- `GetLeftHandType` `VC`
- `GetLevelRequirement` `VC`
- `GetMaxStackSize` `VC`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetPrefixClassification` `C`
- `GetRTTIClassInfo` `VC`
- `GetReplacementAnimations` `VC`
- `GetRerollOverrideTable` `C`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetRightHandType` `VC`
- `GetRolloverSize` `V`
- `GetSeedRerolls` `C`
- `GetShouldRenderAcrossPortals` `VC`
- `GetSimpleDescription` `VC`
- `GetSimpleUIDisplayText` `VC`
- `GetSkillAttributes` `VC`
- `GetStackSize` `VC`
- `GetStaticClassInfo` `S`
- `GetStrengthRequirement` `VC`
- `GetSuffixClassification` `C`
- `GetSymbolBitmapName` `VC`
- `GetTotalAttrCostCount` `VC`
- `GetTransmute` `VC`
- `GetUIBitmapOverlay` `VC`
- `GetUIBitmapText` `VC`
- `GetUIDisplayText` `VC`
- `GetUIGameDescription` `VC`
- `GetUIQualityDescription` `VC`
- `GetUIRequirementText` `VC`
- `GetVisiblePlayer` `C`
- `HasAffix` `C`
- `HasAscendantBonus` `VC`
- `HasMastery` `VC`
- `HasMatchingAppearance` `VC`
- `HasMatchingBaseTexture` `VC`
- `HasMatchingBumpTexture` `VC`
- `HasMatchingMesh` `VC`
- `HasPetBonus` `VC`
- `IgnoreItemSkillCache` `C`
- `IncludeInMinimap` `VC`
- `IncrementStack` `V`
- `InitialUpdate` `V`
- `InitializeItem` `V`
- `IsCharacterAttributePresent` `VC`
- `IsComplete` `VC`
- `IsDamageTypePresent` `VC`
- `IsDefenseTypePresent` `VC`
- `IsDescriptionVisible` `VC`
- `IsItemAvailable` `C`
- `IsMaleCompatible` `C`
- `IsNewPickup`
- `IsOfInterest` `VC`
- `IsPickupOk`
- `IsReagentCompatible` `C`
- `IsRetaliationPresent` `VC`
- `IsRetaliationTypePresent` `VC`
- `IsSkillAttributePresent` `VC`
- `IsSoulbound` `C`
- `IsSoulboundInDBR` `C`
- `IsStackFull` `VC`
- `IsTransmuteCompatible` `C`
- `IsUntradeable` `C`
- `IsUntradeableInDBR` `C`
- `Item`
- `Load` `V`
- `MeetsRequirements` `C`
- `OccludesPathing` `VC`
- `OnDropped` `V`
- `OnPickup` `V`
- `PassLootFilter` `C`
- `PhysicsPost` `V`
- `PhysicsSetup` `V`
- `PhysicsUpdate` `V`
- `PickSparkleTime`
- `PlayDropSound` `V`
- `PlayDropSoundWorld` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RandomizeItem` `V`
- `ReadReplicationData` `V`
- `ResolveEquationVariable` `VC`
- `RestoreState` `V`
- `SaveState` `VC`
- `SearchText`
- `SetAltPickUpLocation`
- `SetCraftedRandom`
- `SetCraftingDisplay`
- `SetDropSoundToPlay` `V`
- `SetEquippedItems` `V`
- `SetFocusThisFrame` `V`
- `SetGMIEffect`
- `SetGender` `V`
- `SetIgnoreItemSkillCache`
- `SetItemClassification`
- `SetItemReplicaInfo`
- `SetMaxStackSize` `V`
- `SetPickupOk`
- `SetPrefixClassification`
- `SetSoulbound`
- `SetStackSize` `V`
- `SetSuffixClassification`
- `SetUntradeable`
- `SetVisiblePlayer`
- `ShouldHideLocation` `VC`
- `ShouldPreventEasyDrops` `C`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `Sparkle`
- `SuppressTextDirections` `V`
- `ToggleLootBeam`
- `UpdateItemVersion`
- `UpdateMesh`
- `UpdateReplicaInfo` `V`
- `UpdateSelf` `V`
- `Use` `V`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Item` `V`

### `ItemArtifact` (Game.dll, 48)

- `AreRequirementsMet` `VC`
- `AttachItem` `V`
- `ContributeRacialBonusDamage` `VC`
- `ContributeRacialBonusDefense` `VC`
- `DecrementStack` `V`
- `DetachItem` `V`
- `GetArtifactClass` `C`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetBoundUIDisplayText` `VC`
- `GetCharAttributes` `VC`
- `GetConversionAttributes` `VC`
- `GetDefenseAttributes` `VC`
- `GetHintTag` `VC`
- `GetItemCost` `VC`
- `GetItemMarketType` `VC`
- `GetItemType` `VC`
- `GetMaxStackSize` `VC`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetSkillAttributes` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `GetUIRequirementText` `VC`
- `HasMastery` `VC`
- `HasPetBonus` `VC`
- `IncrementStack` `V`
- `InitializeItem` `V`
- `IsCharacterAttributePresent` `VC`
- `IsDamageTypePresent` `VC`
- `IsDefenseTypePresent` `VC`
- `IsRetaliationPresent` `VC`
- `IsRetaliationTypePresent` `VC`
- `IsSkillAttributePresent` `VC`
- `IsStackFull` `VC`
- `ItemArtifact`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemArtifact` `V`

### `ItemArtifactFormula` (Game.dll, 94)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `DecrementStack` `V`
- `GetArtifact` `C`
- `GetArtifactCreateQuantity` `C`
- `GetArtifactDatabase`
- `GetArtifactInfo`
- `GetBitmap` `VC`
- `GetCreationCost` `C`
- `GetForceRelicComplete`
- `GetItemId`
- `GetItemType` `VC`
- `GetMaxStackSize` `VC`
- `GetMaximumCraftable` `V`
- `GetRTTIClassInfo` `VC`
- `GetReagent1` `C`
- `GetReagent1BitmapName` `C`
- `GetReagent1Count` `C`
- `GetReagent1DisplayName` `C`
- `GetReagent1Id` `C`
- `GetReagent1QuantityForFormula` `C`
- `GetReagent1QuantityForReroll` `C`
- `GetReagent2` `C`
- `GetReagent2BitmapName` `C`
- `GetReagent2Count` `C`
- `GetReagent2DisplayName` `C`
- `GetReagent2Id` `C`
- `GetReagent2QuantityForFormula` `C`
- `GetReagent2QuantityForReroll` `C`
- `GetReagent3` `C`
- `GetReagent3BitmapName` `C`
- `GetReagent3Count` `C`
- `GetReagent3DisplayName` `C`
- `GetReagent3Id` `C`
- `GetReagent3QuantityForFormula` `C`
- `GetReagent3QuantityForReroll` `C`
- `GetReagent4` `C`
- `GetReagent4BitmapName` `C`
- `GetReagent4Count` `C`
- `GetReagent4DisplayName` `C`
- `GetReagent4Id` `C`
- `GetReagent4QuantityForFormula` `C`
- `GetReagent4QuantityForReroll` `C`
- `GetReagent5` `C`
- `GetReagent5BitmapName` `C`
- `GetReagent5Count` `C`
- `GetReagent5DisplayName` `C`
- `GetReagent5Id` `C`
- `GetReagent5QuantityForFormula` `C`
- `GetReagent5QuantityForReroll` `C`
- `GetReagent6` `C`
- `GetReagent6BitmapName` `C`
- `GetReagent6Count` `C`
- `GetReagent6DisplayName` `C`
- `GetReagent6Id` `C`
- `GetReagent6QuantityForFormula` `C`
- `GetReagent6QuantityForReroll` `C`
- `GetReagentBase` `C`
- `GetReagentBaseBitmapName` `C`
- `GetReagentBaseCount` `C`
- `GetReagentBaseDisplayName` `C`
- `GetReagentBaseId` `C`
- `GetReagentBaseName` `C`
- `GetReagentBaseQuantityForFormula` `C`
- `GetRerollCost` `V`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `IncrementStack` `V`
- `InitialUpdate` `V`
- `IsBluePrintValid` `VC`
- `IsStackFull` `VC`
- `IsValidArtifact` `VC`
- `ItemArtifactFormula`
- `Load` `V`
- `LoadArtifact` `VC`
- `LoadReagent1` `C`
- `LoadReagent2` `C`
- `LoadReagent3` `C`
- `LoadReagent4` `C`
- `LoadReagent5` `C`
- `LoadReagent6` `C`
- `LoadReagentBase` `C`
- `LoadReagents`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ResolveEquationVariable` `VC`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemArtifactFormula` `V`

### `ItemAscensionFormula` (Game.dll, 15)

- `GetAscendedName`
- `GetRTTIClassInfo` `VC`
- `GetRandomizerName`
- `GetStaticClassInfo` `S`
- `IsBluePrintValid` `VC`
- `IsValidArtifact` `VC`
- `ItemAscensionFormula`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemAscensionFormula` `V`

### `ItemAttributeReset` (Game.dll, 19)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `ItemAttributeReset`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemAttributeReset` `V`

### `ItemCharm` (Game.dll, 11)

- `GetHintTag` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `ItemCharm`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemCharm` `V`

### `ItemDevotionReset` (Game.dll, 19)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `ItemDevotionReset`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemDevotionReset` `V`

### `ItemDifficultyUnlock` (Game.dll, 21)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetDifficulty`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `ItemDifficultyUnlock`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ResolveEnum_GameMode` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemDifficultyUnlock` `V`

### `ItemEnchantment` (Game.dll, 47)

- `AreRequirementsMet` `VC`
- `CanBeUsedOn` `VC`
- `ContributeRacialBonusDamage` `VC`
- `ContributeRacialBonusDefense` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetAttachPointName` `C`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetBoundUIDisplayText` `VC`
- `GetCharAttributes` `VC`
- `GetConversionAttributes` `VC`
- `GetDefenseAttributes` `VC`
- `GetHintTag` `VC`
- `GetItemCost` `VC`
- `GetItemType` `VC`
- `GetLongFXName` `C`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetParentItem` `C`
- `GetRTTIClassInfo` `VC`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetRoundFXName` `C`
- `GetSkillAttributes` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `GetUIRequirementText` `VC`
- `InitializeItem` `V`
- `InstallOnCharacter` `VC`
- `ItemEnchantment`
- `ItemUseFilter` `VC`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RemoveFromCharacter` `VC`
- `SetEnchantmentLevel`
- `SetParentItem`
- `UseOn` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kNameTag` `S`
- `kUnusableTag` `S`
- `kUsableTag` `S`
- `~ItemEnchantment` `V`

### `ItemEquipment` (Game.dll, 108)

- `AddAscendantBonus`
- `AddEnchantment`
- `AddEnchantment`
- `AddRelic`
- `AddRelic`
- `AddTransmute`
- `AddTransmute` `V`
- `AreRequirementsMet` `VC`
- `AttachItem` `V`
- `CompareDPS` `C`
- `ContributeRacialBonusDamage` `VC`
- `ContributeRacialBonusDefense` `VC`
- `CreateCharPenaltyReductionRangeText` `C`
- `CreateCharRangeText` `C`
- `CreateConversionRangeText` `C`
- `CreateDamageRangeText` `C`
- `CreateDefenseRangeText` `C`
- `CreateItemPetBonus`
- `CreateItemRacialBonus`
- `CreateRetaliationRangeText` `C`
- `CreateSkillRangeText` `C`
- `CreateUIAttributeText` `VC`
- `DecrementStack` `V`
- `DetachItem` `V`
- `DumpCostAttributes` `V`
- `FastUpdate` `V`
- `GetAlternateMeshName` `VC`
- `GetAttachPointName` `C`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetCharAttributes` `VC`
- `GetConversionAttributes` `VC`
- `GetDefenseAttributes` `VC`
- `GetDexterityRequirement` `VC`
- `GetEnchantment` `C`
- `GetIntelligenceRequirement` `VC`
- `GetItemCost` `VC`
- `GetItemSetName` `C`
- `GetLevelRequirement` `VC`
- `GetMaxStackSize` `VC`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetProtectionRegion` `C`
- `GetRTTIClassInfo` `VC`
- `GetRelic` `C`
- `GetReplacementAnimations` `VC`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetSearchText` `C`
- `GetSkillAttributes` `VC`
- `GetStaticClassInfo` `S`
- `GetStrengthRequirement` `VC`
- `GetTotalAttrCostCount` `VC`
- `GetTransmute` `VC`
- `GetUIBitmapOverlay` `VC`
- `GetUIDisplayText` `VC`
- `GetUIDisplayText_DPS` `C`
- `GetUIDisplayText_OffhandDPS` `C`
- `GetUIDisplayText_PetBonus` `C`
- `GetUIDisplayText_RacialBonus` `C`
- `GetUIRequirementText` `VC`
- `HasAscendantBonus` `VC`
- `HasEnchantment` `C`
- `HasMastery` `VC`
- `HasMatchingAppearance` `VC`
- `HasPetBonus` `VC`
- `HasRelic` `C`
- `HasTransmute` `C`
- `IncrementStack` `V`
- `InitializeEquipmentSpecial` `V`
- `InitializeItem` `V`
- `IsCharacterAttributePresent` `VC`
- `IsDamageTypePresent` `VC`
- `IsDefenseTypePresent` `VC`
- `IsRetaliationPresent` `VC`
- `IsRetaliationTypePresent` `VC`
- `IsSkillAttributePresent` `VC`
- `IsStackFull` `VC`
- `IsTwoHandedMeleeWeapon` `VC`
- `ItemEquipment`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RemoveAttachments` `V`
- `RemoveEnchantment`
- `RemoveRelic`
- `RemoveTransmute`
- `ResolveEquationVariable` `VC`
- `SetDexterityEquation`
- `SetEquippedItems` `V`
- `SetFocusThisFrame` `V`
- `SetGender` `V`
- `SetIntelligenceEquation`
- `SetItemCostEquation`
- `SetLevelRequirementEquation`
- `SetStrengthEquation`
- `ShouldHideLocation` `VC`
- `UpdateFX`
- `UpdateReplicaInfo` `V`
- `UpdateSelf` `V`
- `UpdateTransmute` `V`
- `UseAlternateMesh` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemEquipment` `V`

### `ItemFactionBooster` (Game.dll, 20)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetFactionType`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `ItemFactionBooster`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemFactionBooster` `V`

### `ItemFactionWarrant` (Game.dll, 13)

- `AllowUse` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `ItemFactionWarrant`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemFactionWarrant` `V`

### `ItemHookPack` (Game.dll, 6)

- `ItemHookPack`
- `ItemHookPack`
- `LoadHooks` `V`
- ``vftable'`
- `operator=`
- `~ItemHookPack` `V`

### `ItemNote` (Game.dll, 24)

- `AllowUse` `VC`
- `CanBePlacedInTransferStash` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetCodexSubHeadingTag` `C`
- `GetCodexTitleTag` `C`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRolloverSize` `V`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `InitialUpdate` `V`
- `ItemNote`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemNote` `V`

### `ItemRandomSetFormula` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `ItemRandomSetFormula`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemRandomSetFormula` `V`

### `ItemRelic` (Game.dll, 67)

- `AddToRelicLevel` `V`
- `AreRequirementsMet` `VC`
- `CanAutoPickup` `VC`
- `CanRelicBeUsedOn` `VC`
- `ContributeRacialBonusDamage` `VC`
- `ContributeRacialBonusDefense` `VC`
- `CreateSecondaryCursorHandler` `V`
- `DecrementStack` `V`
- `GetAutoPickupRadius` `VC`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetBoundUIDisplayText` `VC`
- `GetCharAttributes` `VC`
- `GetCompletionLevel` `VC`
- `GetConversionAttributes` `VC`
- `GetDefenseAttributes` `VC`
- `GetHintTag` `VC`
- `GetItemCost` `VC`
- `GetItemType` `VC`
- `GetMaxStackSize` `VC`
- `GetOffensiveDamageAttributes` `VC`
- `GetOffensiveModifierAttributes` `VC`
- `GetParentItem` `C`
- `GetRTTIClassInfo` `VC`
- `GetRelicLevel` `VC`
- `GetRelicOverlayBitmap` `VC`
- `GetRetaliationAttributes` `VC`
- `GetRetaliationModifierAttributes` `VC`
- `GetSkillAttributes` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `GetUIRequirementText` `VC`
- `HasMastery` `VC`
- `HasPetBonus` `VC`
- `IncrementStack` `V`
- `InitializeItem` `V`
- `InstallOnCharacter` `C`
- `IsCharacterAttributePresent` `VC`
- `IsComplete` `VC`
- `IsDamageTypePresent` `VC`
- `IsDefenseTypePresent` `VC`
- `IsRetaliationPresent` `VC`
- `IsRetaliationTypePresent` `VC`
- `IsSkillAttributePresent` `VC`
- `IsStackFull` `VC`
- `ItemRelic`
- `ItemUseFilter` `VC`
- `Load` `V`
- `MakeComplete` `V`
- `OnPickup` `V`
- `PlayCompleteSound`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReloadOverlayBitmap` `C`
- `RemoveFromCharacter` `C`
- `SetParentItem`
- `SetRelicLevel` `V`
- `TestCompatibility` `VC`
- `UpdateSelf` `V`
- `UseRelicOn` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~ItemRelic` `V`

### `ItemReplicaInfo` (Game.dll, 3)

- `ReadProperties`
- `StreamProperties`
- `WriteProperties` `C`

### `ItemRerollFormula` (Game.dll, 13)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBluePrintValid` `VC`
- `IsValidArtifact` `VC`
- `ItemRerollFormula`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemRerollFormula` `V`

### `ItemSet` (Game.dll, 18)

- `CreateMemberDisplayName` `VC`
- `GetSetMembers` `C`
- `GetSetName` `C`
- `GetUIDisplayText` `VC`
- `ItemSet`
- `ItemSet`
- `LoadFromDatabase` `V`
- `PetBonusInstall` `C`
- `PetBonusRemove` `C`
- `SetId`
- `SkillAugmentInstall` `C`
- `SkillAugmentRemove` `C`
- `SkillModifierInstall` `C`
- `SkillModifierRemove` `C`
- ``vftable'`
- ``vftable'`
- `operator=`
- `~ItemSet` `V`

### `ItemSetFormula` (Game.dll, 14)

- `GetItemSetNames` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBluePrintValid` `VC`
- `IsValidArtifact` `VC`
- `ItemSetFormula`
- `Load` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemSetFormula` `V`

### `ItemTransmuter` (Game.dll, 31)

- `CanBeUsedOn` `C`
- `CreateSecondaryCursorHandler` `V`
- `DecrementStack` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetItemType` `VC`
- `GetMaxStackSize` `VC`
- `GetRTTIClassInfo` `VC`
- `GetReplacementAnimations` `VC`
- `GetStaticClassInfo` `S`
- `GetTransmute` `VC`
- `GetTransmuteTag` `C`
- `GetUIDisplayText` `VC`
- `IncrementStack` `V`
- `InitializeItem` `V`
- `IsComplete` `VC`
- `IsStackFull` `VC`
- `IsTransmuteRemover` `C`
- `ItemTransmuter`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ShouldHideLocation` `VC`
- `UseOn`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemTransmuter` `V`

### `ItemTransmuterSet` (Game.dll, 25)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `DecrementStack` `V`
- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetMaxStackSize` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `IncrementStack` `V`
- `InitializeItem` `V`
- `IsComplete` `VC`
- `IsStackFull` `VC`
- `ItemTransmuterSet`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ItemTransmuterSet` `V`

### `JoinPartyConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `JoinPartyConfigCmdPacket`
- `JoinPartyConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~JoinPartyConfigCmdPacket` `V`

### `JointDescription` (Engine.dll, 3)

- `JointDescription`
- `operator=`
- `operator=`

### `JoystickEvent` (Engine.dll, 5)

- `ConvertToButtonEvent` `C`
- `ConvertToMouseEvent` `C`
- `JoystickEvent`
- `operator=`
- `operator=`

### `Jukebox` (Engine.dll, 37)

- `AddStatisticText`
- `AddTrackedBoss`
- `AreTrackedBossesDead` `C`
- `Clean`
- `GetTrackedBosses` `C`
- `GetTrackedPlayer` `C`
- `IsPlayingBossMusic` `C`
- `IsTrackedBoss`
- `IsTrackedPlayerDead` `C`
- `Jukebox`
- `Jukebox`
- `LoadAmbientPlaylist`
- `LoadMusicPlaylist`
- `PlayBossMusic`
- `PlayEventMusic`
- `RemoveTrackedBoss`
- `SetDebug`
- `SetTrackedBossesDead`
- `SetTrackedPlayer`
- `SetTrackedPlayerDead`
- `ShouldPlayMusic` `C`
- `StartAmbient`
- `StartMusic`
- `Stop`
- `StopAmbient`
- `StopBossMusic`
- `StopEventMusic`
- `StopMusic`
- `TrackBossDeathSpawner` `C`
- `Update`
- ``vftable'`
- `kAmbientSwitchDelay` `S`
- `kCrossFadePeriod` `S`
- `kDayNightChangeDelay` `S`
- `kMusicSwitchDelay` `S`
- `operator=`
- `~Jukebox` `V`

### `JumpAttackAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `JumpAttackAction`
- `JumpAttackAction`
- `ToString` `VC`
- ``vftable'`
- `~JumpAttackAction` `V`

### `JumpToAttackPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `JumpToAttackPacket`
- `JumpToAttackPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~JumpToAttackPacket` `V`

### `Level` (Engine.dll, 76)

- `AddEntity`
- `AddEntityToNetworkList`
- `AddReflectionPlanes` `C`
- `AddToScene`
- `CheckLOS` `C`
- `CreatePathMesh`
- `CreatePhysics`
- `DestroyAllEntities`
- `DestroyPhysics`
- `GetAllIntersections` `C`
- `GetBoundingBox` `C`
- `GetDecalSet`
- `GetDecalSet` `C`
- `GetEditableTerrain`
- `GetEditableTerrain` `C`
- `GetEntities` `C`
- `GetEntities` `C`
- `GetEntitiesAroundRay` `C`
- `GetEntitiesInBox` `C`
- `GetEntitiesInFrustum` `C`
- `GetEntitiesInSphere` `C`
- `GetGrid`
- `GetGrid` `C`
- `GetGroundHeight` `C`
- `GetGroundNormal` `C`
- `GetIntersection` `C`
- `GetNavigationData` `C`
- `GetNavigationDataSize` `C`
- `GetPathableActorIntersection` `C`
- `GetPostLoadCalled` `C`
- `GetRegion` `C`
- `GetSectorLayers`
- `GetStaticBoundingBox` `C`
- `GetTerrain`
- `GetTerrain` `C`
- `GetWater`
- `GetWater` `C`
- `HasEntity` `C`
- `HasLoadedRenderData` `C`
- `IsGroundVisible` `C`
- `IsInNetworkList`
- `IsLoaded` `C`
- `IsUnderground` `C`
- `Level`
- `Load`
- `Load`
- `LoadRenderData`
- `MoveEntity`
- `NewGrid`
- `NewSpace`
- `NewTerrain`
- `NewWater`
- `PickEntities` `C`
- `PickEntity` `C`
- `PickEntity` `C`
- `PostDeviceReset`
- `PostLoad`
- `PostLoadEntities`
- `PreDeviceReset`
- `PreLoad`
- `PreLoadFrustums`
- `RegisterPathableActor`
- `ReloadGenerationDependentData`
- `RemoveEntity`
- `RemoveEntityFromNetworkList`
- `RenderPathMeshes` `C`
- `RenderPathableActors` `C`
- `ResizeGrid`
- `ResizeSectorLayers`
- `RestoreState`
- `Save`
- `SaveState`
- `Unload`
- `UnloadEntities`
- `Update`
- `~Level` `V`

### `LevelLimitSectorData` (Engine.dll, 8)

- `Copy` `V`
- `LevelLimitSectorData`
- `LevelLimitSectorData`
- `LevelLimitSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~LevelLimitSectorData` `V`

### `Light` (Engine.dll, 28)

- `AddToScene` `V`
- `Disable` `V`
- `Enable` `V`
- `GetColor` `C`
- `GetIntersection` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetShouldRenderAcrossPortals` `VC`
- `GetStaticClassInfo` `S`
- `GetType` `C`
- `IncludeInMinimap` `V`
- `InitialUpdate` `V`
- `Light`
- `Load` `V`
- `RTTI_new` `S`
- `SetColor`
- `SetForceShadows`
- `SetIntensity`
- `SetRadius`
- `SetShadowPriority`
- `SetSimpleMode`
- `SetType`
- `UpdateBoundingBox` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Light` `V`

### `LightOfRaMarker` (Game.dll, 9)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LightOfRaMarker`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~LightOfRaMarker` `V`

### `LightRig` (Engine.dll, 14)

- `AddLight`
- `AddToScene`
- `AssignLightGroup`
- `Clear`
- `GetLightGroup` `C`
- `GetSceneRenderablesList`
- `LightRig`
- `LightRig`
- `LightRig`
- `LightRig`
- `ShouldRender` `C`
- `operator=`
- `operator=`
- `~LightRig`

### `Lightning` (Game.dll, 22)

- `Generate`
- `GetFadeMultiplier` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTendrilDest`
- `GetTendrilSrc`
- `GetTravelPercentage`
- `HasReachedTarget`
- `Lightning`
- `Load` `V`
- `RTTI_new` `S`
- `RecursiveGenerate`
- `SetTarget`
- `SetValues`
- `TendrilJitter`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Lightning` `V`

### `Lightning2` (Engine.dll, 34)

- `AddToScene` `V`
- `Generate`
- `GenerateSubTendril`
- `GenerateTendril`
- `GetNumRenderPasses` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetStaticClassInfo` `S`
- `GetTexture` `VC`
- `IsFinished` `C`
- `Lightning2`
- `Load` `V`
- `LogInfo` `VC`
- `RTTI_new` `S`
- `RenderPass` `VC`
- `SetColor`
- `SetEnd`
- `SetFromSky`
- `SetStart`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `lightningLife` `S`
- `maxQuads` `S`
- `texChangePeriodMax` `S`
- `texChangePeriodMin` `S`
- `~Lightning2` `V`

### `LineEffect` (Engine.dll, 27)

- `AddToScene` `V`
- `DoSetPauseAtTarget`
- `GetFadeMultiplier` `VC`
- `GetNumRenderPasses` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetStaticClassInfo` `S`
- `GetTexture` `VC`
- `LineEffect`
- `LogInfo` `VC`
- `RTTI_new` `S`
- `RenderPass` `VC`
- `SetPause`
- `SetShader`
- `SetTexture`
- `SetTopBottomPercent`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~LineEffect` `V`

### `LineEffect2` (Engine.dll, 31)

- `AddToScene` `V`
- `Generate` `V`
- `GetDirection` `VC`
- `GetLength` `VC`
- `GetNumRenderPasses` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetStart` `VC`
- `GetStaticClassInfo` `S`
- `GetTexture` `VC`
- `LineEffect2`
- `Load` `V`
- `LogInfo` `VC`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RenderPass` `VC`
- `SetColor` `V`
- `SetEnd` `V`
- `SetStart` `V`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `maxQuads` `S`
- `~LineEffect2` `V`

### `LinkControlAckPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `LinkControlAckPacket`
- `LinkControlAckPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~LinkControlAckPacket` `V`

### `LinkMTUTestPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `LinkMTUTestPacket`
- `LinkMTUTestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~LinkMTUTestPacket` `V`

### `LoadTable` (Engine.dll, 38)

- `CreateName` `C`
- `CreateObjectFromFile` `C`
- `CreateObjectFromProperty` `C`
- `Deserialize` `V`
- `Deserialize` `V`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetBool` `VC`
- `GetBool` `VC`
- `GetFloat` `VC`
- `GetFloat` `VC`
- `GetInt` `VC`
- `GetInt` `VC`
- `GetNumElementsForField` `VC`
- `GetNumElementsForField` `VC`
- `GetValue` `VC`
- `GetValue` `VC`
- `LoadAnimation` `C`
- `LoadAnimationFromFile` `C`
- `LoadMesh` `C`
- `LoadResourceEffect` `C`
- `LoadShader2` `C`
- `LoadTable`
- `LoadTexture` `C`
- `MakeTokens` `C`
- `Serialize` `VC`
- ``vftable'`
- `~LoadTable` `V`

### `LoadTableBinary` (Engine.dll, 35)

- `Deserialize` `V`
- `GetArrayFloat` `C`
- `GetArrayFloat` `C`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayFloat` `VC`
- `GetArrayInt` `C`
- `GetArrayInt` `C`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayInt` `VC`
- `GetArrayValue` `C`
- `GetArrayValue` `C`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetArrayValue` `VC`
- `GetBool` `C`
- `GetBool` `VC`
- `GetBool` `VC`
- `GetFloat` `VC`
- `GetFloat` `VC`
- `GetInt` `VC`
- `GetInt` `VC`
- `GetNumElementsForField` `VC`
- `GetNumElementsForField` `VC`
- `GetValue` `C`
- `GetValue` `VC`
- `GetValue` `VC`
- `LoadTableBinary`
- `Serialize` `VC`
- ``vftable'`
- `~LoadTableBinary` `V`

### `LocalizationManager` (Engine.dll, 56)

- `AddTagToMap` `V`
- `BuildFinalString`
- `CanBeToken`
- `ClearParamBanks`
- `ClearTagMap`
- `FindParamTypes`
- `GenderizeText` `C`
- `GetCurrentLanguage`
- `GetFlagTextureName` `C`
- `GetFormattedTime`
- `GetInternalParam`
- `GetLanguage` `C`
- `GetLanguageName` `C`
- `GetLanguageString`
- `GetLanguageString`
- `GetLanguageTag`
- `GetLocale` `C`
- `GetLocalizationPath` `C`
- `GetNumLanguages` `C`
- `GetPortableLanguageName` `C`
- `GetSentenceSeperators` `C`
- `GetSpecialFontFolder` `C`
- `GetSupportedLanguageNumber`
- `GetText`
- `GetWordMode` `C`
- `HasDialogForLanguage` `C`
- `InitializeBuiltinLanguages`
- `InitializeLanguage`
- `Instance` `S`
- `IsConsoleTagsFilename` `C`
- `IsEnglish` `C`
- `IsLanguageAvailable`
- `IsLanguageSupported` `C`
- `IsSymbolRecognized`
- `LanguageFailedToLoad` `C`
- `Load` `V`
- `LoadModStrings` `V`
- `LoadTags`
- `LocalizationManager`
- `LocalizationManager`
- `Localize`
- `LocalizeList`
- `LocalizeStripColorTags`
- `LocalizeWithoutParams`
- `LocalizerFormatStrip` `C`
- `ReloadLanguage`
- `ShowTagErrors`
- `TagFoundInMap`
- `ToChar` `S`
- `ToChar` `S`
- `ToWChar` `S`
- `ToWChar` `S`
- ``vftable'`
- `criticalSection` `S`
- `operator=`
- `~LocalizationManager`

### `LongIdleAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `LongIdleAction`
- `LongIdleAction`
- `ResetTransitionTimer`
- `ToString` `VC`
- ``vftable'`
- `~LongIdleAction` `V`

### `LongIdleActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `LongIdleActionPacket`
- `LongIdleActionPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~LongIdleActionPacket` `V`

### `LookAtAction` (Game.dll, 9)

- `Execute` `V`
- `FaceTarget` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `LookAtAction`
- `LookAtAction`
- `ToString` `VC`
- ``vftable'`
- `~LookAtAction` `V`

### `LookAtActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `LookAtActionPacket`
- `LookAtActionPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~LookAtActionPacket` `V`

### `LootBase` (Game.dll, 16)

- `GetLootName` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootBase`
- `LootBase`
- `OverrideTest` `V`
- `RTTI_new` `S`
- `SetAltarEnabled` `V`
- `SetLevel` `V`
- `SetNoBrokenItems` `V`
- `SetRandomizerWeightModifiers` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootBase` `V`

### `LootItemTableRandomizer` (Game.dll, 14)

- `GetBrokenTable`
- `GetPrefixTable`
- `GetRandomizerNames` `V`
- `GetSuffixTable`
- `LoadFromDatabase` `V`
- `LootItemTableRandomizer`
- `LootItemTableRandomizer`
- `OverrideWeightsAndChance` `V`
- `SetNoBrokenItems` `V`
- `SetWeightModifiers` `V`
- `TableIterations` `S`
- ``vftable'`
- `operator=`
- `~LootItemTableRandomizer` `V`

### `LootItemTableRandomizer_Dyn` (Game.dll, 16)

- `GetBrokenTable`
- `GetPrefixTable`
- `GetRandomizerNames` `V`
- `GetRarePrefixTable`
- `GetRareSuffixTable`
- `GetSuffixTable`
- `LoadFromDatabase` `V`
- `LootItemTableRandomizer_Dyn`
- `LootItemTableRandomizer_Dyn`
- `OverrideWeightsAndChance` `V`
- `SetAltarEnabled` `V`
- `SetNoBrokenItems` `V`
- `SetWeightModifiers` `V`
- ``vftable'`
- `operator=`
- `~LootItemTableRandomizer_Dyn` `V`

### `LootItemTable_DynWeight` (Game.dll, 18)

- `GetLootName` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootItemTable_DynWeight`
- `LootItemTable_DynWeight`
- `ProcessTableData`
- `RTTI_new` `S`
- `ResolveEquationVariable` `VC`
- `SetDynamicWeights`
- `SetFreePick`
- `SetLevel` `V`
- `SetValidItemLevel`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootItemTable_DynWeight` `V`

### `LootItemTable_FixedWeight` (Game.dll, 13)

- `GetLootName` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootItemTable_FixedWeight`
- `LootItemTable_FixedWeight`
- `OverrideTest` `V`
- `RTTI_new` `S`
- `TableIterations` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootItemTable_FixedWeight` `V`

### `LootLoader` (Game.dll, 13)

- `GetLootName` `VC`
- `Load` `V`
- `Load` `V`
- `LootLoader`
- `LootLoader`
- `SetAltarEnabled`
- `SetLevel`
- `SetNoBrokenItems`
- `SetRandomizerWeightModifiers` `V`
- `ValidateSelectLootRecursive` `VC`
- ``vftable'`
- `operator=`
- `~LootLoader`

### `LootMasterTable` (Game.dll, 12)

- `GetLootName` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootMasterTable`
- `LootMasterTable`
- `RTTI_new` `S`
- `SetLevel` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootMasterTable` `V`

### `LootRandomizerTable` (Game.dll, 13)

- `GetAllEntries` `C`
- `GetRTTIClassInfo` `VC`
- `GetRandomizerName` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootRandomizerTable`
- `LootRandomizerTable`
- `ManualLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootRandomizerTable` `V`

### `LootRandomizerTable_Dynamic` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetRandomizerName` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootRandomizerTable_Dynamic`
- `LootRandomizerTable_Dynamic`
- `ManualLoad` `V`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootRandomizerTable_Dynamic` `V`

### `LootTable` (Game.dll, 14)

- `GetLootName` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRandomizerName` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `LootTable`
- `LootTable`
- `ManualLoad` `V`
- `RTTI_new` `S`
- `ValidateSelectRandomizerRecursive` `C`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~LootTable` `V`

### `LuaManager` (Engine.dll, 14)

- `CollectGarbage`
- `DumpStack`
- `EnableGarbageCollection`
- `GetState`
- `GetState` `C`
- `Initialize`
- `Load`
- `LogInfo`
- `RegisterForUpdates`
- `RegisterProvider`
- `RunCode`
- `Shutdown`
- `UnregisterForUpdates`
- `Update`

### `MapChunkCallback` (Engine.dll, 6)

- `GetChunkId` `C`
- `MapChunkCallback`
- `MapChunkCallback`
- ``vftable'`
- `operator=`
- `~MapChunkCallback` `V`

### `MarketArmor` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor`
- `MarketArmor`
- ``vftable'`
- `operator=`
- `~MarketArmor` `V`

### `MarketArmor_Chest` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Chest`
- `MarketArmor_Chest`
- ``vftable'`
- `operator=`
- `~MarketArmor_Chest` `V`

### `MarketArmor_Feet` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Feet`
- `MarketArmor_Feet`
- ``vftable'`
- `operator=`
- `~MarketArmor_Feet` `V`

### `MarketArmor_Hands` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Hands`
- `MarketArmor_Hands`
- ``vftable'`
- `operator=`
- `~MarketArmor_Hands` `V`

### `MarketArmor_Head` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Head`
- `MarketArmor_Head`
- ``vftable'`
- `operator=`
- `~MarketArmor_Head` `V`

### `MarketArmor_Legs` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Legs`
- `MarketArmor_Legs`
- ``vftable'`
- `operator=`
- `~MarketArmor_Legs` `V`

### `MarketArmor_Shield` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Shield`
- `MarketArmor_Shield`
- ``vftable'`
- `operator=`
- `~MarketArmor_Shield` `V`

### `MarketArmor_Shoulders` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Shoulders`
- `MarketArmor_Shoulders`
- ``vftable'`
- `operator=`
- `~MarketArmor_Shoulders` `V`

### `MarketArmor_Used` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Used`
- `MarketArmor_Used`
- ``vftable'`
- `operator=`
- `~MarketArmor_Used` `V`

### `MarketArmor_Waist` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketArmor_Waist`
- `MarketArmor_Waist`
- ``vftable'`
- `operator=`
- `~MarketArmor_Waist` `V`

### `MarketC2SPurchaseRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketC2SPurchaseRequestPacket`
- `MarketC2SPurchaseRequestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketC2SPurchaseRequestPacket` `V`

### `MarketC2SSellBackPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketC2SSellBackPacket`
- `MarketC2SSellBackPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketC2SSellBackPacket` `V`

### `MarketC2SUpdateRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketC2SUpdateRequestPacket`
- `MarketC2SUpdateRequestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketC2SUpdateRequestPacket` `V`

### `MarketMiniMart` (Game.dll, 11)

- `AddIfUnique` `V`
- `GetDuplicatesAllowed` `VC`
- `LoadItem` `V`
- `MarketMiniMart`
- `MarketMiniMart`
- `Refresh` `V`
- `SetMarket`
- `UploadItems` `V`
- ``vftable'`
- `operator=`
- `~MarketMiniMart` `V`

### `MarketMisc_Amulet` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Amulet`
- `MarketMisc_Amulet`
- ``vftable'`
- `operator=`
- `~MarketMisc_Amulet` `V`

### `MarketMisc_List` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_List`
- `MarketMisc_List`
- ``vftable'`
- `operator=`
- `~MarketMisc_List` `V`

### `MarketMisc_Medal` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Medal`
- `MarketMisc_Medal`
- ``vftable'`
- `operator=`
- `~MarketMisc_Medal` `V`

### `MarketMisc_Potion` (Game.dll, 10)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Potion`
- `MarketMisc_Potion`
- `UploadItems` `V`
- ``vftable'`
- `operator=`
- `~MarketMisc_Potion` `V`

### `MarketMisc_Ring` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Ring`
- `MarketMisc_Ring`
- ``vftable'`
- `operator=`
- `~MarketMisc_Ring` `V`

### `MarketMisc_Scroll` (Game.dll, 10)

- `GetDuplicatesAllowed` `VC`
- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Scroll`
- `MarketMisc_Scroll`
- ``vftable'`
- `operator=`
- `~MarketMisc_Scroll` `V`

### `MarketMisc_Static` (Game.dll, 10)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Static`
- `MarketMisc_Static`
- `UploadItems` `V`
- ``vftable'`
- `operator=`
- `~MarketMisc_Static` `V`

### `MarketMisc_Used` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketMisc_Used`
- `MarketMisc_Used`
- ``vftable'`
- `operator=`
- `~MarketMisc_Used` `V`

### `MarketS2CPurchaseConfirmationPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketS2CPurchaseConfirmationPacket`
- `MarketS2CPurchaseConfirmationPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketS2CPurchaseConfirmationPacket` `V`

### `MarketS2CPurchaseDeniedPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketS2CPurchaseDeniedPacket`
- `MarketS2CPurchaseDeniedPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketS2CPurchaseDeniedPacket` `V`

### `MarketS2CStatusUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketS2CStatusUpdatePacket`
- `MarketS2CStatusUpdatePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketS2CStatusUpdatePacket` `V`

### `MarketS2CUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MarketS2CUpdatePacket`
- `MarketS2CUpdatePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MarketS2CUpdatePacket` `V`

### `MarketWeapon` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon`
- `MarketWeapon`
- ``vftable'`
- `operator=`
- `~MarketWeapon` `V`

### `MarketWeapon_Axe` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Axe`
- `MarketWeapon_Axe`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Axe` `V`

### `MarketWeapon_Dagger` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Dagger`
- `MarketWeapon_Dagger`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Dagger` `V`

### `MarketWeapon_Mace` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Mace`
- `MarketWeapon_Mace`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Mace` `V`

### `MarketWeapon_Ranged1h` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Ranged1h`
- `MarketWeapon_Ranged1h`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Ranged1h` `V`

### `MarketWeapon_Ranged2h` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Ranged2h`
- `MarketWeapon_Ranged2h`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Ranged2h` `V`

### `MarketWeapon_Scepter` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Scepter`
- `MarketWeapon_Scepter`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Scepter` `V`

### `MarketWeapon_Spear` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Spear`
- `MarketWeapon_Spear`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Spear` `V`

### `MarketWeapon_Staff` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Staff`
- `MarketWeapon_Staff`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Staff` `V`

### `MarketWeapon_Sword` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Sword`
- `MarketWeapon_Sword`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Sword` `V`

### `MarketWeapon_Used1` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Used1`
- `MarketWeapon_Used1`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Used1` `V`

### `MarketWeapon_Used2` (Game.dll, 9)

- `GetMarketType` `VC`
- `GetNumItemsMax` `VC`
- `GetNumItemsMin` `VC`
- `GetTableName` `VC`
- `MarketWeapon_Used2`
- `MarketWeapon_Used2`
- ``vftable'`
- `operator=`
- `~MarketWeapon_Used2` `V`

### `MemoryMappedFile` (Engine.dll, 7)

- `Close`
- `GetLength` `C`
- `MapView`
- `MemoryMappedFile`
- `Open`
- `UnmapView`
- `~MemoryMappedFile` `V`

### `MenuIdleAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `MenuIdleAction`
- `MenuIdleAction`
- `ResetTransitionTimer`
- `ToString` `VC`
- ``vftable'`
- `~MenuIdleAction` `V`

### `Monster` (Game.dll, 149)

- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `AreSkillsEnabled` `C`
- `BillboardPunctuation` `V`
- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `CalculateStun` `V`
- `CapAttackSpeed` `VC`
- `CapRunSpeed` `VC`
- `CapSpellCastSpeed` `VC`
- `CharacterHasDied` `V`
- `CharacterIsDying` `V`
- `ClearAnger`
- `CombatExertInfluenceConfusion` `V`
- `ContributeGameBalanceCharAttributes` `VC`
- `ContributeGameBalanceDefenseAttributes` `VC`
- `ContributeGameBalanceOffensiveDamageAttributes` `VC`
- `ContributeGameBalanceOffensiveModifierAttributes` `VC`
- `ContributeGameBalanceRetaliationAttributes` `VC`
- `ContributeGameBalanceRetaliationModifierAttributes` `VC`
- `ContributeGameBalanceSkillAttributes` `VC`
- `ContributeMutatorCharAttributes` `VC`
- `ContributeMutatorDefenseAttributes` `VC`
- `ContributeMutatorOffensiveDamageAttributes` `VC`
- `ContributeMutatorOffensiveModifierAttributes` `VC`
- `ContributeMutatorRetaliationAttributes` `VC`
- `ContributeMutatorRetaliationModifierAttributes` `VC`
- `ContributeMutatorSkillAttributes` `VC`
- `CreateUINextSummaryText` `VC`
- `CreateUISummaryText` `VC`
- `CrowdAgentCreated` `V`
- `CrowdAgentDepenetrate` `V`
- `CrowdAgentError` `V`
- `CrowdAgentMoved` `V`
- `CrowdAgentUpdate` `V`
- `DeathCanFinish` `VC`
- `DoDistressCall` `V`
- `DropFirmlyAttachedItems`
- `DropIfNotBroken`
- `DropItemFromEquipLocation`
- `DropLooselyAttachedItems`
- `EnableSkills`
- `EnableSpawnAnimation` `V`
- `ForceUpdateResources` `V`
- `GetAlertAnimChance` `C`
- `GetAlertSound`
- `GetAlertSoundChance` `C`
- `GetAmbushDissolveTexture` `C`
- `GetAmbushDissolveTime` `C`
- `GetChallengeArea` `C`
- `GetCharLevelGapFixer` `VC`
- `GetClassification` `C`
- `GetConvertLevel` `VC`
- `GetDeathFromEnemyDelay` `C`
- `GetDeathFromEnemyRange` `C`
- `GetDefenseAttributeCap` `VC`
- `GetDropPerPlayerItems` `C`
- `GetEmoteSound`
- `GetExperienceReward` `V`
- `GetFleeAnimChance` `C`
- `GetFleeSound` `C`
- `GetFleeSoundChance` `C`
- `GetGameDescription` `VC`
- `GetLeader` `VC`
- `GetMonsterClassification` `C`
- `GetMonsterMutators` `C`
- `GetOriginalFaction` `VC`
- `GetOriginalFactionPack` `VC`
- `GetPatrolPoints` `C`
- `GetPetAcknowledgeSound` `C`
- `GetPetAttackSound` `C`
- `GetPoppedOut` `C`
- `GetProxyNumSiblings` `C`
- `GetProxyParentId` `C`
- `GetProxyParentName` `C`
- `GetRTTIClassInfo` `VC`
- `GetRaceText` `VC`
- `GetRallyAnimChance` `C`
- `GetRallySound`
- `GetRallySoundChance` `C`
- `GetRampageSound`
- `GetRampageSoundChance` `C`
- `GetRampageSoundDelay` `C`
- `GetShowStatusWidget` `C`
- `GetSleepAggressionFalloffRate` `VC`
- `GetStaticClassInfo` `S`
- `GetStatusIcon` `C`
- `GetStatusIconRed` `C`
- `GetUIActorDescriptionRedirectId` `C`
- `GetWaitingAnimChance` `C`
- `GetWaitingAnimDelay` `C`
- `GetWaitingAnimSound` `C`
- `GiveExperience` `VC`
- `InitSkillsInController`
- `InitialUpdate` `V`
- `IsTargetable` `VC`
- `JoinMe` `V`
- `Load` `V`
- `Lobotomize`
- `Monster`
- `OnDestroy` `V`
- `OnDifficultyRampLevel`
- `OnGameBalanceLevel`
- `OnKilledPlayer` `V`
- `PlayAmbientSound` `V`
- `PreLoad` `V`
- `PreLoadBossMusic`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RemoveAllPetBanners`
- `RemoveBaseBanner`
- `RemoveControlBanner`
- `RemoveConvertBanner`
- `RequestConversation`
- `RestoreState` `V`
- `SaveState` `VC`
- `SelectAlternativeMeshAndTextures`
- `SetBaseBanner`
- `SetControlBanner`
- `SetConvertBanner`
- `SetLeader`
- `SetLifetime`
- `SetMostHatedEnemy`
- `SetPatrolPoints`
- `SetPetBanner`
- `SetPoppedOut`
- `SetProxyNumSiblings`
- `SetProxyParent`
- `SetTreasureProxy`
- `SetUIActorDescriptionRedirectId`
- `ShouldDropItems` `C`
- `ShouldLoadLoot` `VC`
- `ShouldSaveState` `VC`
- `StartAliveSound`
- `StartDamageEffect` `V`
- `StopAliveSound`
- `StopDamageEffect` `V`
- `TransferAnger` `C`
- `UnJoinLeader` `V`
- `UpdateSelf` `V`
- `UseController`
- `UseController`
- `WriteReplicationData` `V`
- `WriteSimulationInformation` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Monster` `V`

### `MonsterShrine` (Game.dll, 16)

- `AppendDetailMapData` `V`
- `BindToLocalPlayer` `V`
- `GetGameDescription` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `MonsterShrine`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~MonsterShrine` `V`

### `MonsterUseControllerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MonsterUseControllerPacket`
- `MonsterUseControllerPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MonsterUseControllerPacket` `V`

### `MouseEvent` (Engine.dll, 4)

- `ConvertToButtonEvent` `C`
- `MouseEvent`
- `operator=`
- `operator=`

### `MouseWrapper` (Widget.dll, 5)

- `Disable`
- `Enable`
- `GetMousePosition` `C`
- `MouseWrapper`
- `Update`

### `MoveAttackAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `MoveAttackAction`
- `MoveAttackAction`
- `ToString` `VC`
- ``vftable'`
- `~MoveAttackAction` `V`

### `MoveAttackPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `MoveAttackPacket`
- `MoveAttackPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~MoveAttackPacket` `V`

### `MoveToAction` (Game.dll, 8)

- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `MoveToAction`
- `MoveToAction`
- `ToString` `VC`
- ``vftable'`
- `~MoveToAction` `V`

### `MoveToPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `MoveToPacket`
- `MoveToPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~MoveToPacket` `V`

### `Mutator` (Game.dll, 13)

- `GetAffectsPlayer` `C`
- `GetAttributePak` `C`
- `GetInfo` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `Mutator`
- `Mutator`
- `RTTI_new` `S`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Mutator` `V`

### `MutatorRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `MutatorRequestPacket`
- `MutatorRequestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~MutatorRequestPacket` `V`

### `NRLoaderStatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `NRLoaderStatePacket`
- `NRLoaderStatePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~NRLoaderStatePacket` `V`

### `NackEntityPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `NackEntityPacket`
- `NackEntityPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~NackEntityPacket` `V`

### `Name` (Engine.dll, 11)

- `Create` `S`
- `GetDigest` `C`
- `Name`
- `SetDigest`
- `noName` `S`
- `operator unsigned __int64` `C`
- `operator!=` `C`
- `operator<` `C`
- `operator=`
- `operator=`
- `operator==` `C`

### `NameSectorData` (Engine.dll, 8)

- `Copy` `V`
- `NameSectorData`
- `NameSectorData`
- `NameSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~NameSectorData` `V`

### `Names` (Engine.dll, 1)

- `empty` `S`

### `NavBlocker` (Game.dll, 13)

- `GetExtents` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `NavBlocker`
- `RTTI_new` `S`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NavBlocker` `V`

### `NavManager` (Engine.dll, 43)

- `AddData`
- `AddDynamicObstacle`
- `AddDynamicObstacle`
- `AddDynamicObstacle`
- `AddObject`
- `BlockPoint`
- `Cleanup`
- `ClearRequestPointData`
- `CreateNavigationData`
- `DebugHighlightPoly`
- `DebugRender`
- `FillPointSet`
- `FindClosestPointOnPathMesh`
- `FindPath`
- `FindRandomPointInRadius`
- `FindStraightMovePoint`
- `FindStraightMovePointOnSlopes`
- `GetDebug` `C`
- `GetNumBlockedPoints` `C`
- `GetUpdateTime` `C`
- `HasObject`
- `Initialize`
- `IsBlocked` `C`
- `IsNavDataLoaded` `C`
- `IsPointOnPathMesh`
- `MoveObject`
- `NavManager`
- `RemoveData`
- `RemoveDynamicObstacle`
- `RemoveObject`
- `RequestAndBlockNearPoint`
- `RequestAndBlockRandomPointInRadius`
- `RequestAndBlockRandomPointOnRadius`
- `ResetObject`
- `SetDebug`
- `SetDebugReason`
- `SetDefaultConfig`
- `StopObject`
- `Update`
- `VisualizeNavData`
- `kMaxTileCacheMemUsage` `S`
- `operator=`
- `~NavManager`

### `NavMesh` (Engine.dll, 6)

- `NavMesh`
- `SetFaceData`
- `SetIndexData`
- `SetVertexData`
- `operator=`
- `~NavMesh`

### `NavMeshBuilder` (Engine.dll, 13)

- `AddBox`
- `AddQuadXZ`
- `Create`
- `Create`
- `Create`
- `CreateNavMesh` `C`
- `NavMeshBuilder`
- `NavMeshBuilder`
- `NavMeshBuilder`
- `Reset`
- `operator=`
- `operator=`
- `~NavMeshBuilder`

### `NemesisSpawnPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `NemesisSpawnPacket`
- `NemesisSpawnPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~NemesisSpawnPacket` `V`

### `NetPacket` (Engine.dll, 20)

- `CheckObjects` `V`
- `CopyInbound` `V`
- `CreateOutBuffer`
- `CreatePacket`
- `FinishObjectProcessing` `V`
- `GetOutBuffer` `V`
- `GetOutBufferSize` `V`
- `GetPacketClass` `C`
- `GetPacketDescription` `V`
- `GetReceiveBuffer`
- `GetReceiveBufferSize`
- `NetPacket`
- `NetPacket`
- `PrepareOutBuffer` `V`
- `SetHostID`
- `SetNetPacketCreator`
- `SetReceiveBuffer`
- ``vftable'`
- `operator=`
- `~NetPacket` `V`

### `NetPacketDescriber` (Engine.dll, 25)

- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `Describe`
- `GetDescription`
- `Heading`
- `InsertDescription`
- `NetPacketDescriber`
- `NetPacketDescriber`
- `NetPacketDescriber`
- `operator=`
- `operator=`
- `~NetPacketDescriber`

### `NetPacketHeader` (Engine.dll, 9)

- `Deserialize`
- `GetAuthenticationOffset`
- `GetAuthenticationSize`
- `NetPacketHeader`
- `NetPacketHeader`
- `Serialize`
- `Size`
- `operator=`
- `~NetPacketHeader`

### `NetPacketInBuffer` (Engine.dll, 17)

- `NetPacketInBuffer`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove`
- `Remove16Bit`
- `Remove16Bit`
- `RemoveRaw`
- `operator=`
- `operator=`

### `NetPacketOutBuffer` (Engine.dll, 23)

- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add`
- `Add16Bit`
- `Add16Bit`
- `AddFileName`
- `AddRaw`
- `Done`
- `NetPacketOutBuffer`
- `NetPacketOutBuffer`
- `NetPacketOutBuffer`
- `operator=`
- `operator=`
- `~NetPacketOutBuffer`

### `NetworkAddress` (Engine.dll, 7)

- `GetNetworkIT` `C`
- `NetworkAddress`
- `NetworkAddress`
- `NetworkAddress`
- ``vftable'`
- `operator=`
- `operator=`

### `NetworkControllerBase` (Engine.dll, 7)

- `IsCommunciationsEnabled`
- `NetworkControllerBase`
- `NetworkControllerBase`
- `NetworkControllerBase`
- ``vftable'`
- `operator=`
- `operator=`

### `NetworkEntityList` (Engine.dll, 32)

- `ActivateClient` `V`
- `AddEntity` `V`
- `AddEntityToAllClients` `V`
- `ClearDebugEntityList`
- `CreateNewClient` `V`
- `CreateNewClient` `V`
- `DebugDumpDiscrepancies`
- `DebugRender`
- `DebugValidateClient`
- `DoesEntityExistOnAnyClient` `V`
- `GetClient` `V`
- `GetClientByIndex` `C`
- `GetClientFrustumList` `V`
- `GetDebugEntityList` `C`
- `GetLocalOperationTimeout`
- `GetNumClients` `C`
- `HandleCreatedEntity` `V`
- `IsClientActive`
- `IsEntityExistOnClient` `V`
- `NetworkEntityList`
- `NetworkEntityList`
- `RemoveAllClients` `V`
- `RemoveClient` `V`
- `RemoveEntity` `V`
- `RemoveEntityFromAllClients` `V`
- `RemoveTimeout` `V`
- `TestListValidity`
- `Update` `V`
- `UpdateFrustum` `V`
- ``vftable'`
- `operator=`
- `~NetworkEntityList` `V`

### `NetworkLinkStats` (Engine.dll, 27)

- `AddPacketCounts` `C`
- `AddReceivePacketCounts` `C`
- `AddTransmitPacketCounts` `C`
- `DumpStats`
- `DumpStatsToString`
- `GetNumberOfErrors` `C`
- `GetReceiveByteCount` `C`
- `GetReceivePacketCount` `C`
- `GetTransmitByteCount` `C`
- `GetTransmitPacketCount` `C`
- `InsertField`
- `InsertField`
- `InsertField`
- `InsertField`
- `LogDuplicatePacketReceived`
- `LogPing`
- `LogRetransmit`
- `LogRetransmitRequest`
- `LogSocketError`
- `NetworkLinkStats`
- `NetworkLinkStats`
- `ProcessReceive`
- `ProcessTransmit`
- `SafeDivide`
- ``vftable'`
- `operator=`
- `~NetworkLinkStats` `V`

### `NetworkQueue` (Engine.dll, 12)

- `DecrementFreeSpace`
- `GetDataSize`
- `GetSpaceFree`
- `IncrementFreeSpace`
- `Insert`
- `NetworkQueue`
- `NetworkQueue`
- `Peek`
- `PeekHead`
- `Remove`
- `operator=`
- `~NetworkQueue`

### `NetworkRateCounter` (Engine.dll, 8)

- `GetDepth`
- `GetRate`
- `NetworkRateCounter`
- `NetworkRateCounter`
- `ProcessPacket`
- ``vftable'`
- `operator=`
- `~NetworkRateCounter` `V`

### `NetworkServerBrowser` (Engine.dll, 33)

- `AddFilter`
- `AddServer`
- `CancelServerListUpdate`
- `ClearServerList`
- `CreateImplementation`
- `GetMyPublicIP` `C`
- `GetPing` `C`
- `GetPlayerLevel`
- `GetServer`
- `GetServer`
- `GetServerList`
- `GetTotalServerCount`
- `InitializeInternetBrowser`
- `InitializeLANBrowser`
- `IsBrowserEnabled`
- `IsInServerList`
- `IsServerListUpdating`
- `NetworkServerBrowser`
- `NetworkServerBrowser`
- `PingResponse`
- `RemoveServer`
- `ScrubServerList`
- `ServerListUpdateComplete`
- `SetBrowsingMode`
- `SetServerListTimeout`
- `ShutdownInternetBrowser`
- `ShutdownLANBrowser`
- `Size`
- `Update`
- `UpdateServerList`
- ``vftable'`
- `operator=`
- `~NetworkServerBrowser` `V`

### `NetworkShim` (Engine.dll, 10)

- `DumpPacketStatsToFile`
- `LogCharacterPacket`
- `NetworkShim`
- `NetworkShim`
- `SendCharacterAction` `V`
- `SendConfigCommand` `V`
- `SetNetworkStats`
- ``vftable'`
- `operator=`
- `~NetworkShim` `V`

### `NetworkSocket` (Engine.dll, 12)

- `Bind` `V`
- `Connect` `V`
- `Disconnect` `V`
- `DumpInterfaceInfo` `V`
- `GetLocalAddress` `VC`
- `GetRemoteAddress` `VC`
- `GetSocketHandle` `C`
- `NetworkSocket`
- `NetworkSocket`
- `SetSocketHandle`
- ``vftable'`
- `~NetworkSocket` `V`

### `NoiseTexture` (Engine.dll, 21)

- `GetNeedsUpdate` `C`
- `GetRTTIClassInfo` `VC`
- `GetRenderSurface` `C`
- `GetSize` `C`
- `GetStaticClassInfo` `S`
- `GetTexture` `C`
- `InitalizeSurface`
- `IsNormalMap` `C`
- `Load` `V`
- `NoiseTexture`
- `NoiseTexture`
- `RTTI_new` `S`
- `ReleaseRenderSurface`
- `SetNeedsUpdate` `C`
- `SetShaderParams` `C`
- `UpdateLayers`
- ``vftable'`
- `classInfo` `S`
- `numLayers` `S`
- `operator=`
- `~NoiseTexture` `V`

### `Npc` (Game.dll, 81)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `BillboardPunctuation` `V`
- `CanBeAttracted` `C`
- `Converse` `V`
- `CreateController`
- `CrowdAgentCreated` `V`
- `CrowdAgentDepenetrate` `V`
- `CrowdAgentUpdate` `V`
- `DebugRender` `V`
- `DeleteSocialTarget` `V`
- `DestroyFunctor`
- `DestroyFunctor`
- `DoDebugRender` `VC`
- `ForcedUpdate` `V`
- `GetAttractionTime` `C`
- `GetBoatMaster` `C`
- `GetChatEnable` `V`
- `GetChatTarget` `V`
- `GetDialogFade` `C`
- `GetDistanceToPlayer` `C`
- `GetMaxRotationSpeed` `C`
- `GetMinRotationSpeed` `C`
- `GetPlayerNpcDialog` `C`
- `GetPlayerNpcIllumination` `C`
- `GetProfessionTag` `C`
- `GetRTTIClassInfo` `VC`
- `GetRolloverDescription` `VC`
- `GetSocialTarget` `V`
- `GetStaticClassInfo` `S`
- `HasConversation` `C`
- `IdleBetweenWaypoints` `C`
- `IncludeInMap` `C`
- `InitialUpdate` `V`
- `IsAlerted` `C`
- `IsAvailableForConversations` `C`
- `IsBoatMaster` `C`
- `IsChatting` `V`
- `IsChattingWithPlayer` `C`
- `IsInHerd` `C`
- `IsNpcAttractor` `C`
- `IsOfInterest` `VC`
- `IsPlayerNpcDialog` `C`
- `IsPlayerNpcIllumination` `C`
- `Load` `V`
- `LoadController` `V`
- `Npc`
- `OnConversationEnd`
- `OnDebugRenderClick`
- `OnPlayerInteract` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RestoreInteractState`
- `RestoreState` `V`
- `SaveState` `VC`
- `SayGoodbye` `V`
- `SayHello` `V`
- `SetBoatMaster`
- `SetBoatMasterPunctuation`
- `SetChatEnable` `V`
- `SetController`
- `SetDirty`
- `SetDispenseItem`
- `SetIgnoreAttractionTime`
- `SetIsAvailabeForConversations`
- `SetVisibility` `V`
- `SetVisibility` `V`
- `ShouldLoadLoot` `VC`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- `UpdateSocialTargetList` `V`
- `UseExistingObjectForRestore` `VC`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Npc` `V`

### `NpcCaravan` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcCaravan`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcCaravan` `V`

### `NpcCauldron` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcCauldron`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcCauldron` `V`

### `NpcCrafter` (Game.dll, 20)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetCrafterBitmapName` `C`
- `GetEnhancementTableName` `C`
- `GetEnhancementTags` `C`
- `GetRTTIClassInfo` `VC`
- `GetRecipes` `C`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `LoadAwakenedRecipes` `C`
- `NpcCrafter`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- `RestrictsRecipes` `C`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcCrafter` `V`

### `NpcEnchanter` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcEnchanter`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcEnchanter` `V`

### `NpcItemAscension` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcItemAscension`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcItemAscension` `V`

### `NpcMerchant` (Game.dll, 20)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `Converse` `V`
- `GetMarketName` `C`
- `GetMerchantType` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcMerchant`
- `OnAddToLevel` `V`
- `OnPlayerInteract` `V`
- `OnRemoveFromLevel` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcMerchant` `V`

### `NpcSkillReallocator` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcSkillReallocator`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcSkillReallocator` `V`

### `NpcTalkPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `NpcTalkPacket`
- `NpcTalkPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~NpcTalkPacket` `V`

### `NpcTransmuter` (Game.dll, 14)

- `AddSocialTarget` `V`
- `AppendDetailMapData` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsOfInterest` `VC`
- `Load` `V`
- `NpcTransmuter`
- `OnPlayerInteract` `V`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcTransmuter` `V`

### `NpcWanderPoint` (Game.dll, 13)

- `GetIntersection` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `NpcWanderPoint`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~NpcWanderPoint` `V`

### `Object` (Engine.dll, 17)

- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `GetObjectId` `C`
- `GetObjectName` `C`
- `GetObjectNameHash` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `Object`
- `Object`
- `RTTI_new` `S`
- `SetObjectId`
- `SetObjectName`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Object` `V`

### `ObjectContainerPacket` (Engine.dll, 6)

- `CheckObjects` `V`
- `ObjectContainerPacket`
- `ObjectContainerPacket`
- ``vftable'`
- `operator=`
- `~ObjectContainerPacket` `V`

### `ObjectInteractionPacket` (Engine.dll, 8)

- `CheckObjects` `V`
- `FinishObjectProcessing` `V`
- `ObjectInteractionPacket`
- `ObjectInteractionPacket`
- `RegisterId`
- ``vftable'`
- `operator=`
- `~ObjectInteractionPacket` `V`

### `ObjectManager` (Engine.dll, 25)

- `CacheObjectEx`
- `ClearRecordData`
- `CreateObjectFromFile`
- `CreateObjectFromFilePartial`
- `CreateObjectID`
- `DestroyObjectEx`
- `DestroyPendingObjects`
- `DumpDetailedObjectList`
- `DumpObjectList`
- `EnableDebugging`
- `FinishCreateObjectFromFilePartial`
- `GetLoadTable` `C`
- `GetNumDeletedObjects` `C`
- `GetNumObjects` `C`
- `GetObjectList` `C`
- `IsObjectIdOnDeletedList`
- `IsObjectOnDeletedList`
- `LoadObjectData`
- `LoadTableFile`
- `LogLeakedObjects`
- `ObjectManager`
- `Release`
- `SetPanicThresholdAndRate`
- `TableDepotDumpStats`
- `~ObjectManager`

### `OneShot` (Game.dll, 24)

- `GetBitmap` `VC`
- `GetBitmapName` `VC`
- `GetBonus` `C`
- `GetButtonDownBitmapName` `C`
- `GetButtonUpBitmapName` `C`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUseDelayTime` `VC`
- `GetUseSound` `V`
- `InitialUpdate` `V`
- `Load` `V`
- `OnPickup` `V`
- `OneShot`
- `PreLoad` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot` `V`

### `OneShot_Dye` (Game.dll, 14)

- `CreateSecondaryCursorHandler` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `Load` `V`
- `OneShot_Dye`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_Dye` `V`

### `OneShot_EndlessDungeon` (Game.dll, 14)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `OneShot_EndlessDungeon`
- `RTTI_new` `S`
- `ShouldSaveState` `VC`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_EndlessDungeon` `V`

### `OneShot_Food` (Game.dll, 19)

- `CanAutoPickup` `VC`
- `GetAutoPickupRadius` `VC`
- `GetGameDescription` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIGameDescription` `VC`
- `InitializeItem` `V`
- `OnPickup` `V`
- `OneShot_Food`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~OneShot_Food` `V`

### `OneShot_Gold` (Game.dll, 20)

- `CanAutoPickup` `VC`
- `GetAutoPickupRadius` `VC`
- `GetGameDescription` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIGameDescription` `VC`
- `InitializeItem` `V`
- `OnPickup` `V`
- `OneShot_Gold`
- `RTTI_new` `S`
- `SetGoldValue` `V`
- `UpdateSelf` `V`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~OneShot_Gold` `V`

### `OneShot_InstantReward` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OneShot_InstantReward`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_InstantReward` `V`

### `OneShot_Potion` (Game.dll, 19)

- `CanAutoPickup` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetAutoPickupRadius` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `InitializeItem` `V`
- `OnPickup` `V`
- `OneShot_Potion`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~OneShot_Potion` `V`

### `OneShot_PotionHealth` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnPickup` `V`
- `OneShot_PotionHealth`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_PotionHealth` `V`

### `OneShot_PotionMana` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnPickup` `V`
- `OneShot_PotionMana`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_PotionMana` `V`

### `OneShot_Sack` (Game.dll, 12)

- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OneShot_Sack`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_Sack` `V`

### `OneShot_Scroll` (Game.dll, 15)

- `CreateSecondaryCursorHandler` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetType` `C`
- `GetUIDisplayText` `VC`
- `Load` `V`
- `OneShot_Scroll`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_Scroll` `V`

### `OneShot_Skill` (Game.dll, 16)

- `CanAutoPickup` `VC`
- `GetAutoPickupRadius` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OneShot_Skill`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~OneShot_Skill` `V`

### `OneShot_SkillUnlock` (Game.dll, 17)

- `AllowUse` `VC`
- `CreateSecondaryCursorHandler` `V`
- `GetRTTIClassInfo` `VC`
- `GetSkillLevelUnlock` `C`
- `GetSkillUnlocks` `C`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `Load` `V`
- `OneShot_SkillUnlock`
- `RTTI_new` `S`
- `Use` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OneShot_SkillUnlock` `V`

### `Options` (Engine.dll, 22)

- `GetBool` `C`
- `GetDevice` `C`
- `GetFloat` `C`
- `GetInt` `C`
- `GetInt2` `C`
- `GetQuality` `C`
- `GetString` `C`
- `Load`
- `LoadDefaults`
- `Options`
- `PrintToConsole`
- `Save`
- `SetBool`
- `SetDevice`
- `SetFloat`
- `SetInt`
- `SetInt2`
- `SetLanguageOptionsToSystemDefaults`
- `SetMenuSceneToDefault`
- `SetQuality`
- `SetString`
- `SetToDefaults`

### `Ormenos` (Game.dll, 14)

- `CharacterIsDying` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `Ormenos`
- `RTTI_new` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `UnequipAndExplodeSickle`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Ormenos` `V`

### `OrmenosDropZone` (Game.dll, 14)

- `GetIntersection` `C`
- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `OccludesPathing` `VC`
- `OrmenosDropZone`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~OrmenosDropZone` `V`

### `OverlayActivity` (Engine.dll, 6)

- `Finished`
- `OverlayActivity`
- `OverlayActivity`
- ``vftable'`
- `operator=`
- `~OverlayActivity`

### `OverlayActivityManager` (Engine.dll, 7)

- `AddActivity`
- `Clear`
- `OverlayActivityManager`
- `RemoveActivity`
- `Render` `C`
- `Update`
- `~OverlayActivityManager`

### `PFxManager` (Game.dll, 19)

- `AddMeshEffect`
- `AddParticleEffect`
- `AttachActor`
- `CalculateAllocatedMemory` `C`
- `OneShotParticleEffect`
- `PFxManager`
- `PFxManager`
- `PreLoad`
- `RemoveMeshEffect`
- `RemoveParticleEffect`
- `SetParticleVisibility`
- `StartMeshEffect`
- `StartParticleEffect`
- `StopMeshEffect`
- `StopParticleEffect`
- `Update`
- ``vftable'`
- `operator=`
- `~PFxManager` `V`

### `PIXEvent` (Engine.dll, 2)

- `PIXEvent`
- `~PIXEvent`

### `Paperdoll` (Game.dll, 30)

- `AttachItem`
- `Cleanup`
- `CopyItem` `C`
- `Create` `S`
- `Destroy` `S`
- `GetAttachPointName` `C`
- `GetCharacter` `C`
- `GetHandState` `C`
- `GetPaperdollRegion` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadOverrideAnimSet`
- `Paperdoll`
- `RTTI_new` `S`
- `SetCharacter`
- `SetItemAppearance`
- `SetSnapshot`
- `SetupDefaultEquip`
- `SyncItems`
- `SyncMesh`
- `TakeSnapshot`
- `UpdateAnimations`
- `UpdateEquipVisibility`
- `UpdateIdleAnimation`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Paperdoll` `V`

### `ParametersCombat` (Game.dll, 12)

- `GetDamage` `V`
- `GetDamage` `VC`
- `GetRetaliationDamage` `V`
- `GetRetaliationDamage` `VC`
- `IsDisplayCombat` `V`
- `ParametersCombat`
- `ParametersCombat`
- `ParametersCombat`
- ``vftable'`
- `operator=`
- `operator=`
- `~ParametersCombat`

### `ParametersCombatDisplay` (Game.dll, 12)

- `GetDamage` `V`
- `GetDamage` `VC`
- `GetRetaliationDamage` `V`
- `GetRetaliationDamage` `VC`
- `IsDisplayCombat` `V`
- `ParametersCombatDisplay`
- `ParametersCombatDisplay`
- `ParametersCombatDisplay`
- ``vftable'`
- `operator=`
- `operator=`
- `~ParametersCombatDisplay`

### `PartyManager` (Game.dll, 26)

- `AreInPartyTogether` `C`
- `AttemptToKickFromLocalParty`
- `CleanUpForPlayerLeave`
- `CreateResources`
- `DestroyResources`
- `GetNumInPlayersParty`
- `GetNumPlayersInParty`
- `GetNumPlayersInPartyNear`
- `GetParty` `C`
- `GetPartyExperienceMultiplier`
- `GetPartyMembers` `C`
- `GetPlayerFaction`
- `GetPlayersInParty` `C`
- `HandlePartyInvite`
- `IsNotInParty` `C`
- `IsPartyLeader` `C`
- `MessagePlayerInvited`
- `MessagePlayerJoin`
- `MessagePlayerLeaving`
- `OnInviteToParty`
- `PartyManager`
- `PartyManager`
- `ReassignRemainingParty`
- `Update`
- `operator=`
- `~PartyManager`

### `PartyRequestResponsePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PartyRequestResponsePacket`
- `PartyRequestResponsePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PartyRequestResponsePacket` `V`

### `PatrolPoint` (Game.dll, 13)

- `GetRTTIClassInfo` `VC`
- `GetRadius` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `PatrolPoint`
- `RTTI_new` `S`
- `ShouldRunTo` `C`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PatrolPoint` `V`

### `Pet` (Game.dll, 37)

- `ContributeGameBalanceCharAttributes` `VC`
- `ContributeGameBalanceDefenseAttributes` `VC`
- `ContributeGameBalanceOffensiveDamageAttributes` `VC`
- `ContributeGameBalanceOffensiveModifierAttributes` `VC`
- `ContributeGameBalanceRetaliationAttributes` `VC`
- `ContributeGameBalanceRetaliationModifierAttributes` `VC`
- `ContributeGameBalanceSkillAttributes` `VC`
- `ContributeMiscCharAttributes` `VC`
- `ContributeMiscConversionAttributes` `VC`
- `ContributeMiscDefenseAttributes` `VC`
- `ContributeMiscOffensiveDamageAttributes` `VC`
- `ContributeMiscOffensiveModifierAttributes` `VC`
- `ContributeMiscRetaliationAttributes` `VC`
- `ContributeMiscRetaliationModifierAttributes` `VC`
- `ContributeMiscSkillAttributes` `VC`
- `ContributeMutatorCharAttributes` `VC`
- `ContributeMutatorDefenseAttributes` `VC`
- `ContributeMutatorOffensiveDamageAttributes` `VC`
- `ContributeMutatorOffensiveModifierAttributes` `VC`
- `ContributeMutatorRetaliationAttributes` `VC`
- `ContributeMutatorRetaliationModifierAttributes` `VC`
- `ContributeMutatorSkillAttributes` `VC`
- `GetCharLevelGapFixer` `VC`
- `GetPetMutators` `C`
- `GetPetTypeNames`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `Pet`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Pet` `V`

### `PetNonScaling` (Game.dll, 16)

- `ContributeGameBalanceCharAttributes` `VC`
- `ContributeGameBalanceDefenseAttributes` `VC`
- `ContributeGameBalanceOffensiveDamageAttributes` `VC`
- `ContributeGameBalanceOffensiveModifierAttributes` `VC`
- `ContributeGameBalanceRetaliationAttributes` `VC`
- `ContributeGameBalanceRetaliationModifierAttributes` `VC`
- `ContributeGameBalanceSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PetNonScaling`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PetNonScaling` `V`

### `PetPen` (Game.dll, 1)

- `GetPetOwner` `C`

### `PetPlayerScaling` (Game.dll, 17)

- `ContributeMiscCharAttributes` `VC`
- `ContributeMiscConversionAttributes` `VC`
- `ContributeMiscDefenseAttributes` `VC`
- `ContributeMiscOffensiveDamageAttributes` `VC`
- `ContributeMiscOffensiveModifierAttributes` `VC`
- `ContributeMiscRetaliationAttributes` `VC`
- `ContributeMiscRetaliationModifierAttributes` `VC`
- `ContributeMiscSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PetPlayerScaling`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PetPlayerScaling` `V`

### `PhysicsDecoration` (Game.dll, 15)

- `CollisionCallback` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `PhysicsDecoration`
- `PhysicsSetup` `V`
- `PhysicsSync` `V`
- `RTTI_new` `S`
- `TakeAttack`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PhysicsDecoration` `V`

### `PhysicsEngine2` (Engine.dll, 17)

- `AddObject`
- `AddToSimulation`
- `BeginUpdate`
- `CreatePhysicsMesh`
- `DebugRender`
- `EndUpdate`
- `GetUpdateTime` `C`
- `Initialize`
- `IsUpdating` `C`
- `PhysicsEngine2`
- `RemoveFromSimulation`
- `RemoveObject`
- `SetDebug`
- `SetPaused`
- `Shutdown`
- `kGravity` `S`
- `~PhysicsEngine2`

### `PhysicsMesh` (Engine.dll, 17)

- `DebugRender` `C`
- `DebugRender` `VC`
- `GetAllocatedSize` `VC`
- `GetBoundingBox` `VC`
- `GetFace` `VC`
- `GetIntersection` `C`
- `GetIntersection` `VC`
- `GetNumFaces` `VC`
- `GetNumVertices` `VC`
- `GetVertex` `VC`
- `Initialize` `V`
- `InitializeBoundingBox`
- `PhysicsMesh`
- `PhysicsMesh`
- ``vftable'`
- `operator=`
- `~PhysicsMesh` `V`

### `PhysicsMeshBase` (Engine.dll, 8)

- `DecRefCount` `C`
- `GetRefCount` `C`
- `IncRefCount` `C`
- `PhysicsMeshBase`
- `PhysicsMeshBase`
- ``vftable'`
- `operator=`
- `~PhysicsMeshBase` `V`

### `PhysicsMesh_Bullet` (Engine.dll, 18)

- `DebugRender` `VC`
- `GetAllocatedSize` `VC`
- `GetBoundingBox` `VC`
- `GetCollisionShape` `C`
- `GetFace` `VC`
- `GetIntersection` `C`
- `GetIntersection` `VC`
- `GetMeshData` `C`
- `GetNumFaces` `VC`
- `GetNumVertices` `VC`
- `GetVertex` `VC`
- `Initialize` `V`
- `InitializeBoundingBox`
- `PhysicsMesh_Bullet`
- `PhysicsMesh_Bullet`
- ``vftable'`
- `operator=`
- `~PhysicsMesh_Bullet` `V`

### `PhysicsObject2` (Engine.dll, 16)

- `Detach`
- `GetOwner` `C`
- `GetPhysicsSystem` `C`
- `PhysicsCollision`
- `PhysicsGetSurfaceType`
- `PhysicsObject2`
- `PhysicsObject2`
- `PhysicsPost`
- `PhysicsResponse`
- `PhysicsSetup`
- `PhysicsSync`
- `PhysicsTest`
- `PhysicsUpdate`
- ``vftable'`
- `operator=`
- `~PhysicsObject2` `V`

### `PhysicsRigidBody2` (Engine.dll, 9)

- `GetName` `C`
- `GetObject` `C`
- `GetUserdata` `C`
- `PhysicsRigidBody2`
- `PhysicsRigidBody2`
- `SetUserdata`
- ``vftable'`
- `operator=`
- `~PhysicsRigidBody2` `V`

### `PhysicsSystem2` (Engine.dll, 9)

- `GetStepsCompleted` `C`
- `GetUserdata` `C`
- `IsActive` `C`
- `PhysicsSystem2`
- `PhysicsSystem2`
- `SetUserData`
- ``vftable'`
- `operator=`
- `~PhysicsSystem2` `V`

### `PhysicsUtil` (Engine.dll, 14)

- `CalculateFusedBoneCoords` `S`
- `CalculateJointCoords` `S`
- `CalculateRigidBodyCoords` `S`
- `ChangeConetwistBasis` `S`
- `GetDirectionVec` `S`
- `GetEffectEnum` `S`
- `GetElevationVec` `S`
- `GetPushVec` `S`
- `ToJointAxis` `S`
- `ToJointType` `S`
- `ToText` `S`
- `ToText` `S`
- `operator=`
- `operator=`

### `PickUpAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `PickUpAction`
- `PickUpAction`
- `ToString` `VC`
- ``vftable'`
- `~PickUpAction` `V`

### `PickUpItemPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `PickUpItemPacket`
- `PickUpItemPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~PickUpItemPacket` `V`

### `PickupItemConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PickupItemConfigCmdPacket`
- `PickupItemConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PickupItemConfigCmdPacket` `V`

### `PieOmatic` (Engine.dll, 10)

- `FadeIn`
- `FadeOut`
- `FlipSegment`
- `PieOmatic`
- `PieOmatic`
- `Render` `C`
- `SetWedgeMode`
- `Update`
- `operator=`
- `~PieOmatic`

### `PingPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PingPacket`
- `PingPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PingPacket` `V`

### `PlayAnimationAction` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `PlayAnimationAction`
- `PlayAnimationAction`
- `ToString` `VC`
- ``vftable'`
- `~PlayAnimationAction` `V`

### `PlayAnimationActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `PlayAnimationActionPacket`
- `PlayAnimationActionPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~PlayAnimationActionPacket` `V`

### `PlayEffectDeathHandler` (Game.dll, 8)

- `EnableAttach`
- `Execute` `V`
- `IsOverideAllowed` `VC`
- `PlayEffectDeathHandler`
- `PlayEffectDeathHandler`
- ``vftable'`
- `operator=`
- `~PlayEffectDeathHandler` `V`

### `PlaySoundConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `PlaySoundConfigCmd`
- `PlaySoundConfigCmd`
- ``vftable'`
- `operator=`
- `~PlaySoundConfigCmd` `V`

### `PlaySoundConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PlaySoundConfigCmdPacket`
- `PlaySoundConfigCmdPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PlaySoundConfigCmdPacket` `V`

### `PlaySoundDeathHandler` (Game.dll, 7)

- `Execute` `V`
- `IsOverideAllowed` `VC`
- `PlaySoundDeathHandler`
- `PlaySoundDeathHandler`
- ``vftable'`
- `operator=`
- `~PlaySoundDeathHandler` `V`

### `PlayStats` (Game.dll, 101)

- `AddLogString`
- `AddRealTimeString`
- `CollectedLoreNote`
- `DifficultySkipUsed`
- `DisableShieldBlockChance`
- `Display`
- `Dump`
- `GainExperience`
- `GainLife`
- `GetBonusFormattedTime` `C`
- `GetBonusTime` `C`
- `GetDifficultySkip` `C`
- `GetEndlessEssence` `C`
- `GetEndlessSouls` `C`
- `GetGreatestMonsterKilledLevel` `C`
- `GetGreatestMonsterKilledLifeAndMana` `C`
- `GetGreatestMonsterKilledName` `C`
- `GetGreatestSurvivalScore` `C`
- `GetHitsReceived` `C`
- `GetLastMonsterHit`
- `GetLastMonsterHitBy`
- `GetNumberOfBossKills` `C`
- `GetNumberOfBossKills` `C`
- `GetNumberOfChampionKills` `C`
- `GetNumberOfDeaths` `C`
- `GetNumberOfHeroKills` `C`
- `GetNumberOfHiddenChestsOpened` `C`
- `GetNumberOfItemsCrafted` `C`
- `GetNumberOfLoreNotesCollected` `C`
- `GetNumberOfMonsterKills` `C`
- `GetNumberOfMythicalRelicsCrafted` `C`
- `GetNumberOfOneShotChestsOpened` `C`
- `GetNumberOfRelicsCrafted` `C`
- `GetNumberOfShrinesRestored` `C`
- `GetNumberOfTranscendentRelicsCrafted` `C`
- `GetNumberOfUltimateVeteranBossKills` `C`
- `GetPlayTimeInMinutes` `C`
- `GetScore`
- `GetSkillStat` `C`
- `GetSurvivalBonusMultiplier` `C`
- `GetSurvivalDefensesBuilt` `C`
- `GetSurvivalPowerUpsActivated` `C`
- `GetSurvivalRestarts` `C`
- `GetSurvivalScore` `C`
- `GetSurvivalWaveTier` `C`
- `ICrafted`
- `ICraftedRelic`
- `IMissed`
- `IWasMissed`
- `IncrementBonusTimer`
- `IncrementDeaths`
- `IncrementEndlessSouls`
- `IncrementKills`
- `IncrementLevel`
- `IncrementPotion`
- `IncrementSurvivalMultiplier`
- `InflictCriticalHit`
- `IsSurvivalScoreLocked`
- `Load`
- `LockSurvivalScore`
- `LoseExperience`
- `OnPreRun`
- `OpenedHiddenChest`
- `OpenedOneShotChest`
- `PlayStats`
- `PlayStats`
- `ReadProperties`
- `ReceiveCriticalHit`
- `ReceiveHit`
- `ResetBonusTimer`
- `ResetDeaths`
- `ResetSurvivalMode`
- `ResetSurvivalMultiplier`
- `ResolveEquationVariable` `VC`
- `SetBonusTimer`
- `SetEndlessEssence`
- `SetGreatestMonsterKilledLevel`
- `SetGreatestMonsterKilledLifeAndMana`
- `SetGreatestMonsterKilledName`
- `SetLastMonsterHitByName`
- `SetLastMonsterHitName`
- `SetShieldBlockChance`
- `SetSkillStat`
- `SetSurvivalMultiplier`
- `SetSurvivalRestarts`
- `SetSurvivalWaveTier`
- `SkillUsed`
- `StreamProperties`
- `TallyDamageInflicted`
- `TallyDamageReceived`
- `TallyDamageReduction`
- `TokenGranted`
- `UnlockedDevotionShrine`
- `Update`
- `UpdateBonusTimer`
- `UpdateStrings`
- `UseMana`
- `WriteProperties` `C`
- ``vftable'`
- `operator=`
- `~PlayStats` `V`

### `PlayVideoCommandPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PlayVideoCommandPacket`
- `PlayVideoCommandPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PlayVideoCommandPacket` `V`

### `PlayVideoRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PlayVideoRequestPacket`
- `PlayVideoRequestPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PlayVideoRequestPacket` `V`

### `Player` (Game.dll, 297)

- `AddAether`
- `AddBoatMaster`
- `AddDiscoveredShrineUID`
- `AddDungeonEntranceUID`
- `AddDynamite`
- `AddExpansionState`
- `AddItemToLoreCodex`
- `AddItemToPrivateStash`
- `AddItemToPrivateStash`
- `AddMarkerUID`
- `AddNpcDialog`
- `AddNpcIllumination`
- `AddRespawnUID`
- `AddSack`
- `AddShrineUID`
- `AddTeleportUID`
- `AddToScene` `V`
- `AllocateAllAnimSets` `VC`
- `ApplyReplicationData` `V`
- `Attach` `V`
- `AttachItemAction` `V`
- `AttackTarget`
- `AutoSave`
- `BestowToken`
- `BestowTokenNow`
- `BossMusicStateUpdate`
- `CalculateAttackSpeed`
- `CalculateDamageModifiers`
- `CalculateDamageValues`
- `CalculateDps` `C`
- `CalculatePetCharAttributeBonus`
- `CalculatePetDamageBonus`
- `CalculateRetaliationValues`
- `CalculateSkillDps`
- `CalculateSkillValues`
- `CalculateStun` `V`
- `CanItemBeAutoEquipped` `VC`
- `CanMoveTo` `VC`
- `CancelAcceleratedLifeRegen` `V`
- `CapAttackSpeed` `VC`
- `CapRunSpeed` `VC`
- `CapSpellCastSpeed` `VC`
- `CharacterIsDying` `V`
- `CleanLoreCodex`
- `ClearTarget`
- `ClearTokens`
- `CollectDamageDurationModifiers`
- `CollectDamageModifiers`
- `CollectDamageValues`
- `CollectRetaliationModifiers`
- `CollectRetaliationValues`
- `CompareItems`
- `ContributeGameBalanceCharAttributes` `VC`
- `ContributeGameBalanceDefenseAttributes` `VC`
- `ContributeGameBalanceOffensiveDamageAttributes` `VC`
- `ContributeGameBalanceOffensiveModifierAttributes` `VC`
- `ContributeGameBalanceRetaliationAttributes` `VC`
- `ContributeGameBalanceRetaliationModifierAttributes` `VC`
- `ContributeGameBalanceSkillAttributes` `VC`
- `ContributeMutatorCharAttributes` `VC`
- `ContributeMutatorDefenseAttributes` `VC`
- `ContributeMutatorOffensiveDamageAttributes` `VC`
- `ContributeMutatorOffensiveModifierAttributes` `VC`
- `ContributeMutatorRetaliationAttributes` `VC`
- `ContributeMutatorRetaliationModifierAttributes` `VC`
- `ContributeMutatorSkillAttributes` `VC`
- `CreateCopy`
- `CreateCopy` `C`
- `CreateDisplayPlayer`
- `CreateFxPak` `V`
- `CreateSpawnNetPacket` `V`
- `CrowdAgentCreated` `V`
- `DeleteOnEnteringUnloadedLevel` `VC`
- `Detach` `V`
- `DetachItemAction` `V`
- `DisplayPlayerCopy`
- `DoDebugRender` `VC`
- `DumpExperienceLevels`
- `DumpRespawnIds`
- `DumpTeleportIds`
- `EnableAlternateConfig`
- `FindPath` `C`
- `ForceSave`
- `GetAlternateConfig` `C`
- `GetAlternateConfigEnabled` `C`
- `GetBoatMaster`
- `GetCenterOfMass` `VC`
- `GetClassNameA` `C`
- `GetClassTag` `C`
- `GetClosestRespawnPoint`
- `GetCompatibleItems`
- `GetCompatibleStash`
- `GetConversationStore`
- `GetCurrentAether` `C`
- `GetCurrentCompassState`
- `GetCurrentDynamite` `C`
- `GetDefaultCameraZoom` `C`
- `GetDefenseAttributeCap` `VC`
- `GetDiscoveredShrineUIDs` `C`
- `GetDisplayPlayer` `C`
- `GetDisplayPlayerItemMap`
- `GetDungeonExitForEntranceUID` `C`
- `GetExpansionStatus` `C`
- `GetFootCoords` `V`
- `GetGameDescription` `VC`
- `GetGameInterface`
- `GetGreatestDifficultyCompleted` `C`
- `GetGreatestSurvivalDifficultyCompleted` `C`
- `GetHasBeenInGame` `C`
- `GetIsBackup` `C`
- `GetItemCountInStashes` `VC`
- `GetItemInLoreCodex` `C`
- `GetItemLevel`
- `GetLastChallengeDifficulty` `C`
- `GetLastDifficultyPlayed` `C`
- `GetLastGameModePlayed` `C`
- `GetLootFilter`
- `GetLoreCodex` `C`
- `GetMarkerUIDs` `C`
- `GetMatchingItemId` `C`
- `GetMaximumSacks` `S`
- `GetMenuDescription` `VC`
- `GetNearSpawnCoords` `C`
- `GetNpcDialog`
- `GetNpcIllumination`
- `GetPartyId` `C`
- `GetPetControllerType` `C`
- `GetPlayerCharacterClass` `C`
- `GetPlayerHotSlotCtrl`
- `GetPlayerMutators` `C`
- `GetPlayerName` `C`
- `GetPlayerNameInChar` `C`
- `GetPlayerNetBasicInfo`
- `GetPlayerNetHeartbeatInfo`
- `GetPlayerTexture`
- `GetPrimarySkillId` `C`
- `GetPrivateStash`
- `GetRTTIClassInfo` `VC`
- `GetReflectCap` `VC`
- `GetRolloverDescription` `C`
- `GetSack`
- `GetSecondarySkillId` `C`
- `GetSelectedStashSackNumber` `C`
- `GetShrineUIDs` `C`
- `GetSkillCooldownText`
- `GetSkillCooldownTextTime`
- `GetSkillWindowShowHelp`
- `GetStaticClassInfo` `S`
- `GetSuperDamage`
- `GetSurvivalTokens` `C`
- `GetTeleportColor` `C`
- `GetTeleportEffectName` `C`
- `GetTeleportUIDs` `C`
- `GetTokens`
- `GetTokens` `C`
- `GetUISettings` `C`
- `GetUnarmedAnimSpeed` `C`
- `GetUniqueSaveId` `C`
- `GetVersion` `C`
- `GetWeaponIdsForCallback` `C`
- `GiveArtifactToCharacter` `V`
- `GiveAscendedItemToCharacter` `V`
- `GiveDismantledBonusItemToCharacter` `V`
- `GiveDismantledItemToCharacter` `V`
- `GiveInitialEquipment` `V`
- `GiveItemToCharacter` `V`
- `GiveRecoveredItemToCharacter` `V`
- `GiveRerollItemToCharacter` `V`
- `GiveSetItemToCharacter` `V`
- `GiveTinkeredItemToCharacter` `V`
- `HasDungeonEntranceUID` `C`
- `HasToken`
- `InitialUpdate` `V`
- `IsAlive` `VC`
- `IsBoatMaster` `C`
- `IsCopy` `VC`
- `IsCurrentRespawnPoint` `C`
- `IsDiscoveredShrineUIDKnown` `C`
- `IsHardcore` `C`
- `IsInMainQuest` `C`
- `IsInventorySpaceAvailable` `VC`
- `IsInvincible` `VC`
- `IsMarkerUIDKnown` `C`
- `IsNpcDialog` `C`
- `IsNpcIllumination` `C`
- `IsPlayer` `VC`
- `IsRelicSpaceAvailable` `VC`
- `IsRespawnUIDKnown` `C`
- `IsShrineUIDKnown` `C`
- `IsTeleportUIDKnown` `C`
- `JumpToNextSpawnPoint`
- `Load` `V`
- `LoadNewFormatData`
- `LoadQuestStatesFromFile`
- `LoadTransmutes`
- `LogInventory` `VC`
- `NotifyControllerItemRemovedFromInventory` `V`
- `OnAttack` `V`
- `OnAttacked` `V`
- `OnCreatureDeath`
- `OnDestroy` `V`
- `OnDurationDamage` `V`
- `OnQuestCompleted`
- `OnQuestUpdated`
- `PhysicsPost` `V`
- `PhysicsUpdate` `V`
- `PickNewTexture`
- `PlayAnimation` `V`
- `PlayInvalidEquipmentVox`
- `PlayInventoryFullSound`
- `PlayItemCooldownVox`
- `PlayLockedChestVox`
- `PlayLockedDoorVox`
- `PlayLockedQuestObjectVox`
- `PlayLockedShrineVox`
- `PlayNotEnoughManaVox`
- `PlaySkillCooldownVox`
- `PlaySound`
- `Player`
- `PostPetSpawn` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadNewFormatHeader` `S`
- `ReadReplicationData` `V`
- `RegisterCombatTextCrit` `V`
- `RegisterCombatTextHit` `V`
- `RegisterMonsterMusic`
- `RemoveItemFromPrivateSacks`
- `RemoveItemFromPrivateStash`
- `RemoveLight`
- `RemoveNpcDialog`
- `RemoveNpcIllumination`
- `RemoveToken`
- `RemoveTokenNow`
- `RequestCompleteInventoryRelics` `C`
- `ResetDisplayPlayer`
- `ResetPlayerTexture`
- `ResetSkillVox`
- `RestoreNumberOfSacks`
- `RestoreUISettings`
- `SaveFOW`
- `SaveNewFormatData` `C`
- `SaveQuestStatesToFile`
- `SendCameraShakeEvent` `V`
- `SetAlternateConfig`
- `SetCompassState`
- `SetDebugSphereRadius`
- `SetEquipmentReplication`
- `SetExpansionStatus`
- `SetGameInterface`
- `SetGreatestDifficultyCompleted`
- `SetGreatestSurvivalDifficultyCompleted`
- `SetHasBeenInGame`
- `SetIgnoreRequirements`
- `SetIsBackup`
- `SetIsControllingCharacter` `V`
- `SetIsHardcore`
- `SetIsInMainQuest`
- `SetLastAttackerId` `V`
- `SetLootFilter`
- `SetLootFilterDefaults`
- `SetMainPlayer`
- `SetPetControllerType`
- `SetPlayerCharacterClass`
- `SetPlayerCharacterClass`
- `SetPlayerName`
- `SetPlayerTexture`
- `SetPrimarySkillId`
- `SetSecondarySkillId`
- `SetSelectedStashSackNumber`
- `SetSkillCooldownText`
- `SetSkillWindowShowHelp`
- `SetSuperDamage`
- `ShouldLoadLoot` `VC`
- `ShouldServerSpawn` `VC`
- `ShouldUseDistressCall` `VC`
- `StopMonsterMusic`
- `SubtractAether`
- `SubtractDynamite`
- `TakeItemFromPrivateStash`
- `TakeItemFromPrivateStash`
- `TakeItemFromStashes` `V`
- `TrackerDumpSessionStats` `C`
- `UpdateIdleAnimation` `V`
- `UpdateReplicationData` `V`
- `UpdateSelf` `V`
- `WalkTo` `V`
- `WriteNewFormatHeader` `C`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `kAutoPickupPeriod` `S`
- `kAutoSavePeriod` `S`
- `kMaxTrackedAttackers` `S`
- `~Player` `V`

### `PlayerHotSlotCtrl` (Game.dll, 63)

- `ActivateEvadeSlot`
- `ActivateHealthPotionSlot`
- `ActivateHotSlot`
- `ActivateManaPotionSlot`
- `ActivatePrimarySlot`
- `ActivateSecondarySlot`
- `CloneDefaultSlot`
- `CreateHotSlotOption` `C`
- `DeactivateActiveSlot`
- `DelayJoystickMovement`
- `GetActiveSkillSetIndex` `C`
- `GetActiveSlotOption`
- `GetActiveSlotOption`
- `GetDisableHotSlotsOverride`
- `GetDisplayedSkillSet` `C`
- `GetDisplayedSkillSetIndex` `C`
- `GetEvadeSlot` `C`
- `GetEvadeStatus` `C`
- `GetHealthPotionSlot` `C`
- `GetHealthPotionStatus` `C`
- `GetHotSlotOption` `C`
- `GetManaPotionSlot` `C`
- `GetManaPotionStatus` `C`
- `GetPrimarySkillId` `C`
- `GetPrimarySlot` `C`
- `GetPrimaryStatus` `C`
- `GetSecondarySkillId` `C`
- `GetSecondarySlot` `C`
- `GetSecondaryStatus` `C`
- `GetSkillSet` `C`
- `GetSlotStatus` `C`
- `IsAnyActiveTargetingNeeded` `C`
- `IsAnySlotActive` `C`
- `IsEvadeSlotActive` `C`
- `IsHealthPotionSlotActive` `C`
- `IsHotSlotActive` `C`
- `IsInDefaultState` `C`
- `IsJoystickMovementAllowed` `C`
- `IsManaPotionSlotActive` `C`
- `IsPrimarySlotActive` `C`
- `IsSecondarySlotActive` `C`
- `NextDisplayedSkillSet`
- `PlayerHotSlotCtrl`
- `PlayerHotSlotCtrl`
- `ReadProperties`
- `Reset`
- `ResetDisplayedSkillSet`
- `SetActiveSkillSet`
- `SetDisableHotSlotsOverride`
- `SetHotSlot`
- `SetHotSlot`
- `SetPlayer`
- `SetPreventJoystickMovement`
- `SetPreventMovement`
- `SetPrimarySkillId`
- `SetPrimarySlot`
- `SetSecondarySkillId`
- `SetSecondarySlot`
- `SetToDefaults`
- `Update`
- `WriteProperties` `C`
- `kDefaultSkillSet` `S`
- `~PlayerHotSlotCtrl`

### `PlayerInfo` (Game.dll, 4)

- `Clear`
- `PlayerInfo`
- `operator=`
- `operator=`

### `PlayerInventoryCtrl` (Game.dll, 76)

- `AddItem`
- `AddItem`
- `AddItemToSack`
- `AddItemsCompatibleWithEnchant`
- `AddItemsCompatibleWithRelic`
- `AddRelicsCompatibleWithItem`
- `AddSack`
- `AddToOneShotMap`
- `AlignRect`
- `ClearItemsCompatibleWithEnchant`
- `ClearItemsCompatibleWithRelic`
- `ClearRelicsCompatibleWithItem`
- `CreatePotionMap`
- `DepositReagents`
- `GetButtonDownBitmapName` `C`
- `GetButtonUpBitmapName` `C`
- `GetCompatibleParent` `C`
- `GetConflicts` `C`
- `GetCurrentCooldown` `C`
- `GetCurrentCooldown` `C`
- `GetCurrentScrollCooldown` `C`
- `GetFocusSackNumber` `C`
- `GetInventoryInSack` `C`
- `GetItemLocation` `C`
- `GetItemUnderPoint` `C`
- `GetNumberOfSacks` `C`
- `GetPotionText` `C`
- `GetPotionTextOfType` `C`
- `GetPotionType` `C`
- `GetRectUnderPoint` `C`
- `GetSack`
- `GetScrollType` `C`
- `GetSelectedSackNumber` `C`
- `GetTotalBonus` `C`
- `GetTotalCooldown` `C`
- `GetTotalCooldown` `C`
- `GetTotalPotions` `C`
- `GetTotalPotions` `C`
- `GetTotalScrollCooldown` `C`
- `GetUniquePotionsOfType` `C`
- `IsItemAddedWhileNotTheCurrentlySelectedInventoryTab` `C`
- `IsItemCompatibleWithEnchants` `C`
- `IsItemCompatibleWithRelic` `C`
- `IsItemInAnySack` `C`
- `IsOneShotReady` `C`
- `IsRelicCompatibleWithItem` `C`
- `IsRelicSpaceAvailable` `C`
- `IsSpaceAvailable` `C`
- `IsSpaceAvailable` `C`
- `MaxSacks` `S`
- `OneShotFilter`
- `PickOneShot`
- `PlayerInventoryCtrl`
- `PlayerInventoryCtrl`
- `ReadProperties`
- `RemoveItem`
- `RemoveItemFromPotionMap`
- `RestoreNumberOfSacks`
- `SetController`
- `SetFocusSackNumber`
- `SetItemAddedWhileNotTheCurrentlySelectedInventoryTab`
- `SetParent`
- `SetSelectedSackNumber`
- `SortPrimarySack`
- `SortSecondarySack`
- `StreamProperties`
- `Update`
- `UseItem`
- `UsePotionOfType`
- `UsePotionOfType`
- `UsesSharedCooldown` `C`
- `WriteProperties` `C`
- ``vftable'`
- `emptyString` `S`
- `operator=`
- `~PlayerInventoryCtrl` `V`

### `PlayerManagerClient` (Game.dll, 36)

- `AddPlayerToParty`
- `BanPlayer`
- `BanPlayer`
- `CheckLatency` `C`
- `Clear`
- `DumpPlayersToConsole`
- `GetAllPlayersInGame` `C`
- `GetAveragePartyLevel` `C`
- `GetMainPlayer` `C`
- `GetPlayerClass` `C`
- `GetPlayerIdFromHostId` `C`
- `GetPlayerInfo`
- `GetPlayerInfo` `C`
- `GetPlayerInfoCache` `C`
- `GetPlayerLevel` `C`
- `GetPlayerLocation` `C`
- `GetPlayerName` `C`
- `GetPlayerPing` `C`
- `GetPlayerWithName` `C`
- `HandleBonus`
- `HandleFaction`
- `HandlePlayerHeartbeatInbound`
- `HandlePlayerHeartbeatOutbound`
- `HandlePlayerUpdate`
- `IsPlayerInGame` `C`
- `IsPlayerInList` `C`
- `PlayerManagerClient`
- `RemovePlayerFromGame`
- `SetClientServicesModule`
- `SetMainPlayer`
- `Update`
- `UpdateGameInfo`
- `UpdateLocalPlayer`
- `VerifyPlayerEquipment`
- ``vftable'`
- `~PlayerManagerClient` `V`

### `PlayerManagerServer` (Game.dll, 25)

- `AddPlayerToParty`
- `Clear`
- `GetExperienceMultiplier` `C`
- `GetNumberOfPlayers` `C`
- `GetPlayerLocation` `C`
- `HandleBonus`
- `HandleFaction`
- `HandleNewPlayer`
- `HandlePlayerHeartbeatInbound`
- `HandlePlayerHeartbeatOutbound`
- `HandlePlayerUpdateOutbound`
- `HandleRemovePlayer`
- `HandleShrineReward`
- `LoadEquations`
- `PlayerManagerServer`
- `PlayerManagerServer`
- `RemovePlayerFromGame`
- `ResolveEquationVariable` `VC`
- `SendExperienceNotification`
- `SendShrineReward`
- `SetServerServicesModule`
- `Update`
- ``vftable'`
- `operator=`
- `~PlayerManagerServer`

### `PlayerNetBasicInfo` (Game.dll, 7)

- `ChecksumInventory`
- `PlayerNetBasicInfo`
- `PlayerNetBasicInfo`
- ``vftable'`
- `operator=`
- `operator=`
- `~PlayerNetBasicInfo` `V`

### `PlayerNetHeartbeatInfo` (Game.dll, 6)

- `PlayerNetHeartbeatInfo`
- `PlayerNetHeartbeatInfo`
- `PlayerNetHeartbeatInfo`
- `operator=`
- `operator=`
- `~PlayerNetHeartbeatInfo`

### `PlayerPositionUpdatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PlayerPositionUpdatePacket`
- `PlayerPositionUpdatePacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PlayerPositionUpdatePacket` `V`

### `PlayerSpawnPoint` (Game.dll, 13)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsStatic` `VC`
- `OccludesPathing` `VC`
- `PlayerSpawnPoint`
- `RTTI_new` `S`
- `ShouldServerSpawn` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PlayerSpawnPoint` `V`

### `PointDisturbance` (Engine.dll, 14)

- `GetIntersection` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PointDisturbance`
- `RTTI_new` `S`
- `ResolveEnum_Mode` `S`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PointDisturbance` `V`

### `PortableLocalization` (Engine.dll, 8)

- `GetText` `C`
- `PortableLocalization`
- `PortableLocalization`
- `Read`
- `Read`
- `noTag` `S`
- `operator=`
- `~PortableLocalization`

### `Portal` (Engine.dll, 29)

- `GetBackToFrontCoords` `C`
- `GetCastsShadows` `C`
- `GetChokePoint` `C`
- `GetConnectedPortal` `C`
- `GetConnectedPortalId` `C`
- `GetConnectedRegion` `C`
- `GetConnectedRegionId` `C`
- `GetCoords` `C`
- `GetFrontToBackCoords` `C`
- `GetId` `C`
- `GetIntersection` `C`
- `GetIsOpen` `C`
- `GetPlane` `C`
- `GetRegion` `C`
- `GetRegionBoundingBox` `C`
- `IsFrontFacing` `C`
- `Portal`
- `Render` `C`
- `SetCastsShadows`
- `SetConnectedPortalId`
- `SetConnectedRegionId`
- `SetCoords`
- `SetId`
- `SetIsOpen`
- `SetLocalChokePoint`
- `SetRegion`
- `SetTriangles`
- `TestIntersection` `C`
- `~Portal` `V`

### `PostPetSpawnPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PostPetSpawnPacket`
- `PostPetSpawnPacket`
- `PrepareOutBuffer` `V`
- ``vftable'`
- `operator=`
- `~PostPetSpawnPacket` `V`

### `Profile` (Engine.dll, 11)

- `BeginFrame` `S`
- `DumpToFile` `S`
- `Enable` `S`
- `EndCycleCount` `S`
- `EndFrame` `S`
- `GetTimeInfo` `S`
- `Profile`
- `Profile`
- `Profile`
- `StartCycleCount` `S`
- `~Profile`

### `ProjectileAreaEffect` (Game.dll, 16)

- `AddToWorld` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PlaySwipeSound` `V`
- `ProcessFriendsInArea`
- `ProjectileAreaEffect`
- `ProjectileGo` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileAreaEffect` `V`

### `ProjectileArrowLike` (Game.dll, 14)

- `CollisionCallback` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Penetrate`
- `ProjectileArrowLike`
- `ProjectileGo` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileArrowLike` `V`

### `ProjectileBase` (Game.dll, 66)

- `AddProjectileModifier` `V`
- `AnimationCallback` `V`
- `ApplyFumbleDamage`
- `ClearProjectileTracking`
- `CollisionCallback` `V`
- `CountWeaponTrails`
- `CreateExploadingImpactFx`
- `CreateImpactFx`
- `DeleteProjectile`
- `DisableWeaponTrail`
- `EnableWeaponTrail`
- `FilterAndAddKnownTargets`
- `FindClosestTarget` `C`
- `GetCollisionType` `VC`
- `GetDistance` `VC`
- `GetFlightCoords` `VC`
- `GetHitPoint`
- `GetHitTime` `VC`
- `GetInfo`
- `GetLaunchAngle` `VC`
- `GetMissTime` `VC`
- `GetPiercingModifier` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetsAlongPath`
- `GetTotalSpeed`
- `IsMaster` `C`
- `Load` `V`
- `LoadInfo` `V`
- `MoveStraightLine`
- `MoveStraightLine`
- `MoveToLocationCommand` `V`
- `MoveTrajectory`
- `MultipleTargetCommand` `V`
- `NotifyMonsters` `V`
- `OccludesPathing` `VC`
- `OnDestroy` `V`
- `PhysicsSetup` `V`
- `PhysicsUpdate` `V`
- `PlayBounceSound`
- `PlayDestructSound`
- `PlayExplodingHitSound`
- `PlayHitSound`
- `PlaySwipeSound` `V`
- `PreLoad` `V`
- `ProcessExplosion` `V`
- `ProcessFragmentation`
- `ProcessPathTargets` `V`
- `ProjectileBase`
- `ProjectileGo` `V`
- `ProjectileStateUpdateCommand` `V`
- `RTTI_new` `S`
- `SetFilteredTargets`
- `SetMaster` `V`
- `SetSkillFlightFx`
- `SingleTargetCommand` `V`
- `SpecialSecondaryActivation`
- `StartFlightAnimation`
- `StartLaunchAnimation`
- `SwitchWeaponTrail`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileBase` `V`

### `ProjectileExploding` (Game.dll, 14)

- `CollisionCallback` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `ProjectileExploding`
- `ProjectileGo` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileExploding` `V`

### `ProjectileFireballLike` (Game.dll, 17)

- `CollisionCallback` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PassThrough` `VC`
- `ProjectileFireballLike`
- `ProjectileGo` `V`
- `ProjectileHit` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `StopAtOriginalTarget` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileFireballLike` `V`

### `ProjectileFragmenting` (Game.dll, 16)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PassThrough` `VC`
- `ProjectileFragmenting`
- `ProjectileGo` `V`
- `ProjectileHit` `V`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `StopAtOriginalTarget` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileFragmenting` `V`

### `ProjectileGrenade` (Game.dll, 17)

- `CollisionCallback` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetStoredOrientation` `V`
- `Load` `V`
- `PhysicsPost` `V`
- `ProjectileGo` `V`
- `ProjectileGrenade`
- `ProjectileStop` `V`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- `UseStoredOrientation` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileGrenade` `V`

### `ProjectileTelekinesis` (Game.dll, 33)

- `CollisionCallback` `V`
- `CreateExplosionEffect` `C`
- `CreatePathObstacle`
- `Explode`
- `FindAndProcessTargets`
- `GetFlightCoords` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsIdle` `C`
- `IsInUse` `VC`
- `Load` `V`
- `MoveToTarget`
- `OccludesPathing` `VC`
- `OnAddToLevel` `V`
- `OnMoveInLevel` `V`
- `OnRemoveFromLevel` `V`
- `PhysicsSetup` `V`
- `PreLoad` `V`
- `ProjectileCome` `V`
- `ProjectileFall` `V`
- `ProjectileGo` `V`
- `ProjectileStop` `V`
- `ProjectileTelekinesis`
- `RTTI_new` `S`
- `Release`
- `RemovePathObstacle`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileTelekinesis` `V`

### `ProjectileTerrainFollowing` (Game.dll, 14)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadInfo` `V`
- `PhysicsSync` `V`
- `PhysicsUpdate` `V`
- `ProjectileGo` `V`
- `ProjectileTerrainFollowing`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProjectileTerrainFollowing` `V`

### `Prop` (Game.dll, 14)

- `AttachProp` `V`
- `DetachProp` `V`
- `GetAttachPoint` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PreLoad` `V`
- `Prop`
- `RTTI_new` `S`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Prop` `V`

### `Proxy` (Game.dll, 49)

- `AccessoryComplete` `V`
- `AddUniqueIdToEntity`
- `ClampedPlayerAverage`
- `DelayedRun`
- `Disable`
- `GetAccessoryObjects`
- `GetAccessoryPoolName` `C`
- `GetIgnoreBossRestriction`
- `GetPlacedObjects` `VC`
- `GetPoolName` `C`
- `GetPrimaryObjects`
- `GetPrimaryPoolName` `C`
- `GetRTTIClassInfo` `VC`
- `GetState` `C`
- `GetStaticClassInfo` `S`
- `GetVisibility` `VC`
- `InitialUpdate` `V`
- `InitializePools` `V`
- `Load` `V`
- `LoadPoolSelection` `VC`
- `OccludesPathing` `VC`
- `OnDestroy` `V`
- `PlaceObjects`
- `PoolComplete` `V`
- `PreLoad` `V`
- `Proxy`
- `RTTI_new` `S`
- `Reset`
- `ResetSettings`
- `ResolveEquationVariable` `VC`
- `RestoreState` `V`
- `RunEquation`
- `RunProxy`
- `SaveState` `VC`
- `SelectPool` `C`
- `SelectPoolLocations`
- `SetDesignerLimits`
- `SetForcedEntityUpdates`
- `SetIgnoreBossRestriction`
- `SetLimitsLocation`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Proxy` `V`

### `ProxyAccessoryPool` (Game.dll, 17)

- `CreateAddToAscendantList`
- `CreateAddToOtherList`
- `GetAscendantObjects`
- `GetFixedItemSelection`
- `GetObjects`
- `GetPoolName` `C`
- `LoadFixedItemSelections`
- `LoadFromFile`
- `PreLoad`
- `ProxyAccessoryPool`
- `ProxyAccessoryPool`
- `RemoveObjectsNotPlaced`
- `RunPool`
- `SetParent`
- ``vftable'`
- `operator=`
- `~ProxyAccessoryPool` `V`

### `ProxyAmbush` (Game.dll, 19)

- `GetPlacedObjects` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsAlert` `C`
- `Load` `V`
- `PlaceNextObject`
- `PoolComplete` `V`
- `ProxyAmbush`
- `RTTI_new` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `ShouldSaveState` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProxyAmbush` `V`

### `ProxyEndless` (Game.dll, 22)

- `GetAccessoryName` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitializeAccessoryPool`
- `InitializePools` `V`
- `IsActive` `C`
- `Load` `V`
- `LoadAccessorySelection` `VC`
- `LoadPoolSelection` `VC`
- `ProxyEndless`
- `RTTI_new` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `SelectAccessory` `C`
- `SetActive`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProxyEndless` `V`

### `ProxyMenu` (Game.dll, 15)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PlaceNextObject`
- `PoolComplete` `V`
- `ProxyMenu`
- `RTTI_new` `S`
- `ShouldSaveState` `VC`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ProxyMenu` `V`

### `PulseLight` (Engine.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PulseLight`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~PulseLight` `V`

### `Punctuation` (Game.dll, 11)

- `EnableOutline` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Punctuation`
- `RTTI_new` `S`
- `SetHighlight` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Punctuation` `V`

### `Puppet` (Game.dll, 19)

- `AnimationCallback` `V`
- `FadeIn` `V`
- `FadeOut` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsReady` `VC`
- `Load` `V`
- `PreLoad` `V`
- `Puppet`
- `RTTI_new` `S`
- `ReleaseToWorld` `V`
- `StartAnimating` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Puppet` `V`

### `PvpSectorData` (Engine.dll, 7)

- `PvpSectorData`
- `PvpSectorData`
- `PvpSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~PvpSectorData` `V`

### `Quest2` (Game.dll, 35)

- `BeginTask`
- `Complete`
- `CompleteTask`
- `Describe`
- `GetFileName` `C`
- `GetFlags` `C`
- `GetGroup` `C`
- `GetHash` `C`
- `GetId` `C`
- `GetName` `C`
- `GetNumInProgressTasks` `C`
- `GetNumTasks` `C`
- `GetTaskByIndex` `C`
- `GetTaskByUid` `C`
- `GetText` `C`
- `InProgress` `C`
- `IsBlocked` `C`
- `IsComplete` `C`
- `IsDirty` `C`
- `IsStarted` `C`
- `IsTracked` `C`
- `Load`
- `MpComplete`
- `Quest2`
- `Quest2`
- `ReadProperties`
- `RegisterTaskRewards`
- `Reset`
- `SetTracked`
- `Update`
- `WriteProperties` `C`
- `kMagic` `S`
- `kVersion` `S`
- `operator=`
- `~Quest2`

### `Quest2Event` (Game.dll, 11)

- `Evaluate` `C`
- `Execute`
- `GetActions` `C`
- `GetConditions` `C`
- `GetText` `C`
- `Quest2Event`
- `Quest2Event`
- `Read`
- ``vftable'`
- `operator=`
- `~Quest2Event` `V`

### `Quest2Objective` (Game.dll, 21)

- `Clone` `C`
- `Evaluate` `C`
- `Execute`
- `GetActions` `C`
- `GetConditions` `C`
- `GetText` `C`
- `GetUid` `C`
- `HasChanged`
- `IsDirty` `C`
- `IsSatisfied` `C`
- `OnChanged`
- `Quest2Objective`
- `Quest2Objective`
- `Read`
- `ReadProperties`
- `SetSatisfied`
- `StreamState`
- `WriteProperties` `C`
- ``vftable'`
- `operator=`
- `~Quest2Objective` `V`

### `Quest2Repository` (Game.dll, 35)

- `AnyoneHasToken`
- `BeginQuestTask`
- `CompleteQuestTask`
- `DebugBeginQuest`
- `DebugCompleteQuest`
- `FreeQuests`
- `GetAllPlayerTokens` `C`
- `GetDebug` `C`
- `GetDisplayName` `C`
- `GetQuest` `C`
- `GetQuest` `C`
- `GetQuestTaskIndexFromUid` `C`
- `GetQuests`
- `Load`
- `LoadQuests`
- `LogQuests`
- `LogQuests`
- `Quest2Repository`
- `ReadProperties`
- `RemovePlayerToken`
- `Reset`
- `ResetAllQuests`
- `ServerHasToken`
- `SetAllPlayerTokens`
- `SetDebug`
- `SetLanguage`
- `SetPlayerToken`
- `SetPlayerTokens`
- `SetServerCompletedBlockerTasks`
- `SetServerCompletedQuests`
- `Update`
- `WriteProperties` `C`
- `kUpdatePeriod` `S`
- `kVersion` `S`
- `~Quest2Repository`

### `Quest2Task` (Game.dll, 41)

- `AdjustRewards`
- `AreObjectivesMet` `C`
- `Complete`
- `Describe`
- `DontPropagate` `C`
- `GetDescription` `C`
- `GetHash` `C`
- `GetIncentives` `C`
- `GetName` `C`
- `GetObjectiveByUid` `C`
- `GetObjectives` `C`
- `GetQuest` `C`
- `GetRewards` `C`
- `GetState` `C`
- `GetTaskFlag` `C`
- `GetUid` `C`
- `HasName` `C`
- `HasRewardActions`
- `HasRewardActions`
- `InProgress` `C`
- `IsAvailable` `C`
- `IsBlocked` `C`
- `IsBlocker` `C`
- `IsComplete` `C`
- `IsComplete` `C`
- `IsDirty` `C`
- `IsObjectiveCondition` `C`
- `IsPointRewardComplete` `C`
- `MpComplete`
- `Quest2Task`
- `Quest2Task`
- `Read`
- `ReadProperties`
- `Reset`
- `Start`
- `StreamState`
- `Update`
- `WasInProgress` `C`
- `WriteProperties` `C`
- `operator=`
- `~Quest2Task`

### `QuestAnimationCompletedConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `QuestAnimationCompletedConfigCmd`
- `QuestAnimationCompletedConfigCmd`
- ``vftable'`
- `operator=`
- `~QuestAnimationCompletedConfigCmd` `V`

### `QuestAnimationCompletedConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestAnimationCompletedConfigCmdPacket`
- `QuestAnimationCompletedConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~QuestAnimationCompletedConfigCmdPacket` `V`

### `QuestCommandBeginQuestTaskPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandBeginQuestTaskPacket`
- `QuestCommandBeginQuestTaskPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandBeginQuestTaskPacket` `V`

### `QuestCommandCompleteQuestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandCompleteQuestPacket`
- `QuestCommandCompleteQuestPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandCompleteQuestPacket` `V`

### `QuestCommandCompleteQuestTaskPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandCompleteQuestTaskPacket`
- `QuestCommandCompleteQuestTaskPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandCompleteQuestTaskPacket` `V`

### `QuestCommandDeclareTokensPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandDeclareTokensPacket`
- `QuestCommandDeclareTokensPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandDeclareTokensPacket` `V`

### `QuestCommandDestroyDestructiblePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandDestroyDestructiblePacket`
- `QuestCommandDestroyDestructiblePacket`
- ``vftable'`
- `operator=`
- `~QuestCommandDestroyDestructiblePacket` `V`

### `QuestCommandEnableMonsterSkillsPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandEnableMonsterSkillsPacket`
- `QuestCommandEnableMonsterSkillsPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandEnableMonsterSkillsPacket` `V`

### `QuestCommandEventPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandEventPacket`
- `QuestCommandEventPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandEventPacket` `V`

### `QuestCommandGiveTokenPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandGiveTokenPacket`
- `QuestCommandGiveTokenPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandGiveTokenPacket` `V`

### `QuestCommandGlobalEventPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandGlobalEventPacket`
- `QuestCommandGlobalEventPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandGlobalEventPacket` `V`

### `QuestCommandLockChestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandLockChestPacket`
- `QuestCommandLockChestPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandLockChestPacket` `V`

### `QuestCommandLockDoorPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandLockDoorPacket`
- `QuestCommandLockDoorPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandLockDoorPacket` `V`

### `QuestCommandMovePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandMovePacket`
- `QuestCommandMovePacket`
- ``vftable'`
- `operator=`
- `~QuestCommandMovePacket` `V`

### `QuestCommandOpenDoorPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandOpenDoorPacket`
- `QuestCommandOpenDoorPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandOpenDoorPacket` `V`

### `QuestCommandOpenDynGridEntrancePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandOpenDynGridEntrancePacket`
- `QuestCommandOpenDynGridEntrancePacket`
- ``vftable'`
- `operator=`
- `~QuestCommandOpenDynGridEntrancePacket` `V`

### `QuestCommandPlayAnimationPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandPlayAnimationPacket`
- `QuestCommandPlayAnimationPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandPlayAnimationPacket` `V`

### `QuestCommandRemoveTokenPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandRemoveTokenPacket`
- `QuestCommandRemoveTokenPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandRemoveTokenPacket` `V`

### `QuestCommandTakeItemPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandTakeItemPacket`
- `QuestCommandTakeItemPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandTakeItemPacket` `V`

### `QuestCommandUiNotifyPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandUiNotifyPacket`
- `QuestCommandUiNotifyPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandUiNotifyPacket` `V`

### `QuestCommandUseSkillPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandUseSkillPacket`
- `QuestCommandUseSkillPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandUseSkillPacket` `V`

### `QuestCommandWalkPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestCommandWalkPacket`
- `QuestCommandWalkPacket`
- ``vftable'`
- `operator=`
- `~QuestCommandWalkPacket` `V`

### `QuestItem` (Game.dll, 23)

- `CanAutoPickup` `VC`
- `CanBePlacedInTransferStash` `VC`
- `GetAutoPickupRadius` `VC`
- `GetBitmap` `VC`
- `GetItemType` `VC`
- `GetQuestVisibility` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIDisplayText` `VC`
- `InitialUpdate` `V`
- `Load` `V`
- `OnPickup` `V`
- `PreLoad` `V`
- `QuestItem`
- `RTTI_new` `S`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `pickupWait` `S`
- `~QuestItem` `V`

### `QuestMessagePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestMessagePacket`
- `QuestMessagePacket`
- ``vftable'`
- `operator=`
- `~QuestMessagePacket` `V`

### `QuestMoveCompletedConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `QuestMoveCompletedConfigCmd`
- `QuestMoveCompletedConfigCmd`
- ``vftable'`
- `operator=`
- `~QuestMoveCompletedConfigCmd` `V`

### `QuestMoveCompletedConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `QuestMoveCompletedConfigCmdPacket`
- `QuestMoveCompletedConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~QuestMoveCompletedConfigCmdPacket` `V`

### `RadiusMagic` (Game.dll, 15)

- `GetCurrentRadius` `C`
- `GetRTTIClassInfo` `VC`
- `GetShaderParam` `VC`
- `GetShaderParamFloat4` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `RadiusMagic`
- `SetEffect`
- `SetEffect`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~RadiusMagic` `V`

### `RandomUniformLocked` (Engine.dll, 5)

- `FGenerate`
- `GenerateSeed`
- `IGenerate`
- `RandomUniformLocked`
- `Seed`

### `ReclaimDevotionPointConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReclaimDevotionPointConfigCmd`
- `ReclaimDevotionPointConfigCmd`
- ``vftable'`
- `operator=`
- `~ReclaimDevotionPointConfigCmd` `V`

### `ReclaimDevotionPointConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReclaimDevotionPointConfigCmdPacket`
- `ReclaimDevotionPointConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReclaimDevotionPointConfigCmdPacket` `V`

### `RefreshSkillCooldownController` (Game.dll, 8)

- `IsCooldownRefresh` `VC`
- `RefreshSkillCooldownController`
- `RefreshSkillCooldownController`
- `RefreshSkillCooldownController`
- ``vftable'`
- `operator=`
- `operator=`
- `~RefreshSkillCooldownController` `V`

### `RefreshSkillDurationController` (Game.dll, 8)

- `IsDurationRefresh` `VC`
- `RefreshSkillDurationController`
- `RefreshSkillDurationController`
- `RefreshSkillDurationController`
- ``vftable'`
- `operator=`
- `operator=`
- `~RefreshSkillDurationController` `V`

### `Region` (Engine.dll, 83)

- `AddEntity`
- `AddToScene`
- `BackgroundLoadLevel`
- `BuildRegionConnectivity`
- `CreatePortal`
- `CreateUniqueId`
- `DestroyLevelEntities`
- `DestroyPortal`
- `GeometryBusStop`
- `GeometryBusStop`
- `GetBoundingBox` `C`
- `GetConnectedRegions` `C`
- `GetEnclosingFrustum` `C`
- `GetEntitiesInFrustum`
- `GetEntitiesInFrustum` `C`
- `GetEntitiesInSphere`
- `GetFogOfWar`
- `GetFramesNotUpdated` `C`
- `GetIcon` `C`
- `GetId`
- `GetId` `C`
- `GetLevel` `C`
- `GetLevelPtr` `C`
- `GetLoadFileName` `C`
- `GetMinimapImage`
- `GetName` `C`
- `GetNumPortals` `C`
- `GetOffsetFromWorld` `C`
- `GetPortal` `C`
- `GetPortal` `C`
- `GetRegionsInFrustum`
- `GetRelativePosition` `C`
- `GetRelativeTransformation` `C`
- `GetShrineRecord` `C`
- `GetSkybox` `C`
- `GetSkyboxRecord` `C`
- `GetStaticBoundingBox` `C`
- `GetTintColor` `C`
- `GetWorldIndex` `C`
- `GetZoneRecord` `C`
- `GuaranteedGetLevel` `C`
- `IsInFog`
- `IsLevelLoaded` `C`
- `IsLevelLoading` `C`
- `IsLoadingFinished` `C`
- `IsNeighbor` `C`
- `IsUnderground` `C`
- `IsUpdating` `C`
- `LoadLevel`
- `LoadRegionIcon`
- `MarkAsUsedThisFrame` `C`
- `PostDeviceReset`
- `PostLoadEntities`
- `PostLoadLevel`
- `PreDeviceReset`
- `PreLoad`
- `PreLoadFrustums`
- `PrepareUnload`
- `RebuildMapData`
- `Region`
- `Region`
- `ReleaseMinimapImage`
- `ReloadGenerationDependentData`
- `RemoveEntity`
- `Save`
- `SaveRegionIcon`
- `SetId`
- `SetLevel`
- `SetMinimapData`
- `SetName`
- `SetOffsetFromWorld`
- `SetShrineRecord`
- `SetSkyboxRecord`
- `SetZoneRecord`
- `TraceSegmentAgainstPortals` `C`
- `UnloadFOW`
- `UnloadLevel`
- `Update`
- `Update`
- `UpdateBoundingBox`
- `UpdateUsage`
- `WaitForLoadingToFinish`
- `~Region` `V`

### `RegionId` (Engine.dll, 12)

- `CreateUnique`
- `GetData` `C`
- `IsInvalid` `C`
- `Read`
- `RegionId`
- `RegionId`
- `Write` `C`
- `Write` `C`
- `operator<` `C`
- `operator=`
- `operator==` `C`
- `~RegionId`

### `RegionLoader` (Engine.dll, 7)

- `GetAreLevelsLoaded` `C`
- `GetFrustum` `C`
- `GetIsDone` `C`
- `RegionLoader`
- `RegionLoader`
- `SetFrustum`
- `Update`

### `ReleaseImmobilizeConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleaseImmobilizeConfigCmd`
- `ReleaseImmobilizeConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleaseImmobilizeConfigCmd` `V`

### `ReleaseImmobilizeConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleaseImmobilizeConfigCmdPacket`
- `ReleaseImmobilizeConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleaseImmobilizeConfigCmdPacket` `V`

### `ReleaseKnockdownConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleaseKnockdownConfigCmd`
- `ReleaseKnockdownConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleaseKnockdownConfigCmd` `V`

### `ReleaseKnockdownConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleaseKnockdownConfigCmdPacket`
- `ReleaseKnockdownConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleaseKnockdownConfigCmdPacket` `V`

### `ReleasePetConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleasePetConfigCmd`
- `ReleasePetConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleasePetConfigCmd` `V`

### `ReleasePetConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleasePetConfigCmdPacket`
- `ReleasePetConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleasePetConfigCmdPacket` `V`

### `ReleaseSleepConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleaseSleepConfigCmd`
- `ReleaseSleepConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleaseSleepConfigCmd` `V`

### `ReleaseSleepConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleaseSleepConfigCmdPacket`
- `ReleaseSleepConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleaseSleepConfigCmdPacket` `V`

### `ReleaseStunConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleaseStunConfigCmd`
- `ReleaseStunConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleaseStunConfigCmd` `V`

### `ReleaseStunConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleaseStunConfigCmdPacket`
- `ReleaseStunConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleaseStunConfigCmdPacket` `V`

### `ReleaseTrapConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ReleaseTrapConfigCmd`
- `ReleaseTrapConfigCmd`
- ``vftable'`
- `operator=`
- `~ReleaseTrapConfigCmd` `V`

### `ReleaseTrapConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ReleaseTrapConfigCmdPacket`
- `ReleaseTrapConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ReleaseTrapConfigCmdPacket` `V`

### `RemoteMessagePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemoteMessagePacket`
- `RemoteMessagePacket`
- ``vftable'`
- `operator=`
- `~RemoteMessagePacket` `V`

### `RemoteWMessagePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemoteWMessagePacket`
- `RemoteWMessagePacket`
- ``vftable'`
- `operator=`
- `~RemoteWMessagePacket` `V`

### `RemoveClientPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemoveClientPacket`
- `RemoveClientPacket`
- ``vftable'`
- `operator=`
- `~RemoveClientPacket` `V`

### `RemoveEntityPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemoveEntityPacket`
- `RemoveEntityPacket`
- ``vftable'`
- `operator=`
- `~RemoveEntityPacket` `V`

### `RemoveInventoryItemConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `RemoveInventoryItemConfigCmd`
- `RemoveInventoryItemConfigCmd`
- ``vftable'`
- `operator=`
- `~RemoveInventoryItemConfigCmd` `V`

### `RemoveInventoryItemConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemoveInventoryItemConfigCmdPacket`
- `RemoveInventoryItemConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~RemoveInventoryItemConfigCmdPacket` `V`

### `RemovePetBonusFxPakPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemovePetBonusFxPakPacket`
- `RemovePetBonusFxPakPacket`
- ``vftable'`
- `operator=`
- `~RemovePetBonusFxPakPacket` `V`

### `RemovePetBonusPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemovePetBonusPacket`
- `RemovePetBonusPacket`
- ``vftable'`
- `operator=`
- `~RemovePetBonusPacket` `V`

### `RemovePetConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `RemovePetConfigCmd`
- `RemovePetConfigCmd`
- ``vftable'`
- `operator=`
- `~RemovePetConfigCmd` `V`

### `RemovePetConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RemovePetConfigCmdPacket`
- `RemovePetConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~RemovePetConfigCmdPacket` `V`

### `RenderDevice` (Engine.dll, 16)

- `Create` `S`
- `Destroy` `S`
- `DeviceLost` `V`
- `FindDeviceLibrary` `S`
- `GetGenerationForName` `S`
- `GetNameForGeneration` `S`
- `ReadFrameBuffer` `VC`
- `RenderDevice`
- `RenderDevice`
- `ResetDevice` `V`
- `Resume` `V`
- `Suspend` `V`
- `UnloadLibraries` `S`
- ``vftable'`
- `operator=`
- `~RenderDevice` `V`

### `RenderDynamicIndexBuffer` (Engine.dll, 5)

- `RenderDynamicIndexBuffer`
- `RenderDynamicIndexBuffer`
- ``vftable'`
- `operator=`
- `~RenderDynamicIndexBuffer` `V`

### `RenderDynamicVertexBuffer` (Engine.dll, 5)

- `RenderDynamicVertexBuffer`
- `RenderDynamicVertexBuffer`
- ``vftable'`
- `operator=`
- `~RenderDynamicVertexBuffer` `V`

### `RenderGeometryShader` (Engine.dll, 5)

- `RenderGeometryShader`
- `RenderGeometryShader`
- ``vftable'`
- `operator=`
- `~RenderGeometryShader` `V`

### `RenderIndexBuffer` (Engine.dll, 5)

- `RenderIndexBuffer`
- `RenderIndexBuffer`
- ``vftable'`
- `operator=`
- `~RenderIndexBuffer` `V`

### `RenderPass` (Engine.dll, 4)

- `RenderPass`
- `RenderPass`
- ``vftable'`
- `~RenderPass` `V`

### `RenderPixelShader` (Engine.dll, 5)

- `RenderPixelShader`
- `RenderPixelShader`
- ``vftable'`
- `operator=`
- `~RenderPixelShader` `V`

### `RenderSurface` (Engine.dll, 5)

- `RenderSurface`
- `RenderSurface`
- ``vftable'`
- `operator=`
- `~RenderSurface` `V`

### `RenderTexture` (Engine.dll, 5)

- `RenderTexture`
- `RenderTexture`
- ``vftable'`
- `operator=`
- `~RenderTexture` `V`

### `RenderVertexBuffer` (Engine.dll, 5)

- `RenderVertexBuffer`
- `RenderVertexBuffer`
- ``vftable'`
- `operator=`
- `~RenderVertexBuffer` `V`

### `RenderVertexDeclaration` (Engine.dll, 5)

- `RenderVertexDeclaration`
- `RenderVertexDeclaration`
- ``vftable'`
- `operator=`
- `~RenderVertexDeclaration` `V`

### `RenderVertexShader` (Engine.dll, 5)

- `RenderVertexShader`
- `RenderVertexShader`
- ``vftable'`
- `operator=`
- `~RenderVertexShader` `V`

### `RequestAllyAttackConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `RequestAllyAttackConfigCmd`
- `RequestAllyAttackConfigCmd`
- ``vftable'`
- `operator=`
- `~RequestAllyAttackConfigCmd` `V`

### `RequestAllyAttackConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RequestAllyAttackConfigCmdPacket`
- `RequestAllyAttackConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~RequestAllyAttackConfigCmdPacket` `V`

### `RequestAllyMoveConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `RequestAllyMoveConfigCmd`
- `RequestAllyMoveConfigCmd`
- ``vftable'`
- `operator=`
- `~RequestAllyMoveConfigCmd` `V`

### `RequestAllyMoveConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RequestAllyMoveConfigCmdPacket`
- `RequestAllyMoveConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~RequestAllyMoveConfigCmdPacket` `V`

### `RequestDungeonDataPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RequestDungeonDataPacket`
- `RequestDungeonDataPacket`
- ``vftable'`
- `operator=`
- `~RequestDungeonDataPacket` `V`

### `RequestResendPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RequestResendPacket`
- `RequestResendPacket`
- ``vftable'`
- `operator=`
- `~RequestResendPacket` `V`

### `RequestSurvivalModeDataPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `RequestSurvivalModeDataPacket`
- `RequestSurvivalModeDataPacket`
- ``vftable'`
- `operator=`
- `~RequestSurvivalModeDataPacket` `V`

### `ResetAttributePointsConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ResetAttributePointsConfigCmd`
- `ResetAttributePointsConfigCmd`
- ``vftable'`
- `operator=`
- `~ResetAttributePointsConfigCmd` `V`

### `ResetAttributesConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ResetAttributesConfigCmdPacket`
- `ResetAttributesConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ResetAttributesConfigCmdPacket` `V`

### `ResetObject` (Engine.dll, 5)

- `ResetObject`
- `ResetObject`
- ``vftable'`
- `operator=`
- `~ResetObject` `V`

### `Resource` (Engine.dll, 28)

- `EnsureAvailable` `C`
- `GetDebugInfo` `VC`
- `GetFileName` `C`
- `GetInLoadingQueue` `C`
- `GetInitResult` `C`
- `GetIsLoaded` `C`
- `GetIsNotUnloaded` `C`
- `GetLastAccessCounter` `C`
- `GetLastFrameUsed` `VC`
- `GetLastTouchedFrame` `C`
- `GetLoadedState` `C`
- `GetResourceLoader`
- `GetResourceManager`
- `GetSystemMemoryUsage` `VC`
- `GetVideoMemoryUsage` `VC`
- `InitializeDefault` `V`
- `IsAvailable` `C`
- `Lock` `C`
- `LogInfo` `VC`
- `MarkAsTouched` `C`
- `MarkAsUsed` `C`
- `PreLoadDependentResources` `V`
- `Resource`
- `SetInitResult`
- `SetLoadedState`
- `SetResourceLoader`
- `Unlock` `C`
- `~Resource` `V`

### `ResourceLoader` (Engine.dll, 23)

- `CreateMarker`
- `EnableDebugging`
- `EnableMainThreadDebugging`
- `EnableSingleProcessorMode`
- `EnqueueResource`
- `GetHasMarkerPast` `C`
- `GetMarkerCount` `C`
- `IsDebugging` `C`
- `IsIdle` `C`
- `IsMainThreadDebugging` `C`
- `LoadResource`
- `Pause`
- `PurgeAllResources`
- `PurgeResource`
- `ResourceLoader`
- `Resume`
- `StartThreads`
- `StopThreads`
- `UnloadResource`
- `Update`
- `WaitForIdle`
- ``default constructor closure'`
- `~ResourceLoader` `V`

### `RespawnAction` (Game.dll, 9)

- `AnimationCallback` `V`
- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `RespawnAction`
- `RespawnAction`
- `ToString` `VC`
- ``vftable'`
- `~RespawnAction` `V`

### `RespawnPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `RespawnPacket`
- `RespawnPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~RespawnPacket` `V`

### `RetaliationAttributeAbs` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `RetaliationAttributeAbs`
- `RetaliationAttributeAbs`
- `RetaliationAttributeAbs`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs` `V`

### `RetaliationAttributeAbsMod_Aether` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Aether`
- `RetaliationAttributeAbsMod_Aether`
- `RetaliationAttributeAbsMod_Aether`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Aether` `V`

### `RetaliationAttributeAbsMod_Chaos` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Chaos`
- `RetaliationAttributeAbsMod_Chaos`
- `RetaliationAttributeAbsMod_Chaos`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Chaos` `V`

### `RetaliationAttributeAbsMod_Cold` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Cold`
- `RetaliationAttributeAbsMod_Cold`
- `RetaliationAttributeAbsMod_Cold`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Cold` `V`

### `RetaliationAttributeAbsMod_DamageMultiplier` (Game.dll, 14)

- `AddModifierToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_DamageMultiplier`
- `RetaliationAttributeAbsMod_DamageMultiplier`
- `RetaliationAttributeAbsMod_DamageMultiplier`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_DamageMultiplier` `V`

### `RetaliationAttributeAbsMod_Elemental` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Elemental`
- `RetaliationAttributeAbsMod_Elemental`
- `RetaliationAttributeAbsMod_Elemental`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Elemental` `V`

### `RetaliationAttributeAbsMod_Fire` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Fire`
- `RetaliationAttributeAbsMod_Fire`
- `RetaliationAttributeAbsMod_Fire`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Fire` `V`

### `RetaliationAttributeAbsMod_Knockdown` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Knockdown`
- `RetaliationAttributeAbsMod_Knockdown`
- `RetaliationAttributeAbsMod_Knockdown`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Knockdown` `V`

### `RetaliationAttributeAbsMod_Life` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Life`
- `RetaliationAttributeAbsMod_Life`
- `RetaliationAttributeAbsMod_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Life` `V`

### `RetaliationAttributeAbsMod_Lightning` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Lightning`
- `RetaliationAttributeAbsMod_Lightning`
- `RetaliationAttributeAbsMod_Lightning`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Lightning` `V`

### `RetaliationAttributeAbsMod_Physical` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Physical`
- `RetaliationAttributeAbsMod_Physical`
- `RetaliationAttributeAbsMod_Physical`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Physical` `V`

### `RetaliationAttributeAbsMod_Pierce` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Pierce`
- `RetaliationAttributeAbsMod_Pierce`
- `RetaliationAttributeAbsMod_Pierce`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Pierce` `V`

### `RetaliationAttributeAbsMod_PierceRatio` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_PierceRatio`
- `RetaliationAttributeAbsMod_PierceRatio`
- `RetaliationAttributeAbsMod_PierceRatio`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_PierceRatio` `V`

### `RetaliationAttributeAbsMod_Poison` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Poison`
- `RetaliationAttributeAbsMod_Poison`
- `RetaliationAttributeAbsMod_Poison`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Poison` `V`

### `RetaliationAttributeAbsMod_Sleep` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Sleep`
- `RetaliationAttributeAbsMod_Sleep`
- `RetaliationAttributeAbsMod_Sleep`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Sleep` `V`

### `RetaliationAttributeAbsMod_Stun` (Game.dll, 13)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_Stun`
- `RetaliationAttributeAbsMod_Stun`
- `RetaliationAttributeAbsMod_Stun`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_Stun` `V`

### `RetaliationAttributeAbsMod_TotalDamageModifier` (Game.dll, 14)

- `AddModifierToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbsMod_TotalDamageModifier`
- `RetaliationAttributeAbsMod_TotalDamageModifier`
- `RetaliationAttributeAbsMod_TotalDamageModifier`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbsMod_TotalDamageModifier` `V`

### `RetaliationAttributeAbs_Aether` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Aether`
- `RetaliationAttributeAbs_Aether`
- `RetaliationAttributeAbs_Aether`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Aether` `V`

### `RetaliationAttributeAbs_Chaos` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Chaos`
- `RetaliationAttributeAbs_Chaos`
- `RetaliationAttributeAbs_Chaos`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Chaos` `V`

### `RetaliationAttributeAbs_Cold` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Cold`
- `RetaliationAttributeAbs_Cold`
- `RetaliationAttributeAbs_Cold`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Cold` `V`

### `RetaliationAttributeAbs_Confusion` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Confusion`
- `RetaliationAttributeAbs_Confusion`
- `RetaliationAttributeAbs_Confusion`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Confusion` `V`

### `RetaliationAttributeAbs_Convert` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Convert`
- `RetaliationAttributeAbs_Convert`
- `RetaliationAttributeAbs_Convert`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Convert` `V`

### `RetaliationAttributeAbs_ElementalDamage` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_ElementalDamage`
- `RetaliationAttributeAbs_ElementalDamage`
- `RetaliationAttributeAbs_ElementalDamage`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_ElementalDamage` `V`

### `RetaliationAttributeAbs_Fear` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Fear`
- `RetaliationAttributeAbs_Fear`
- `RetaliationAttributeAbs_Fear`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Fear` `V`

### `RetaliationAttributeAbs_Fire` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Fire`
- `RetaliationAttributeAbs_Fire`
- `RetaliationAttributeAbs_Fire`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Fire` `V`

### `RetaliationAttributeAbs_Life` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Life`
- `RetaliationAttributeAbs_Life`
- `RetaliationAttributeAbs_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Life` `V`

### `RetaliationAttributeAbs_Lightning` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Lightning`
- `RetaliationAttributeAbs_Lightning`
- `RetaliationAttributeAbs_Lightning`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Lightning` `V`

### `RetaliationAttributeAbs_PercentCurrentLife` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_PercentCurrentLife`
- `RetaliationAttributeAbs_PercentCurrentLife`
- `RetaliationAttributeAbs_PercentCurrentLife`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_PercentCurrentLife` `V`

### `RetaliationAttributeAbs_Physical` (Game.dll, 15)

- `AddDamageToAccumulator` `VC`
- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Physical`
- `RetaliationAttributeAbs_Physical`
- `RetaliationAttributeAbs_Physical`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Physical` `V`

### `RetaliationAttributeAbs_Pierce` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Pierce`
- `RetaliationAttributeAbs_Pierce`
- `RetaliationAttributeAbs_Pierce`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Pierce` `V`

### `RetaliationAttributeAbs_Poison` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeAbs_Poison`
- `RetaliationAttributeAbs_Poison`
- `RetaliationAttributeAbs_Poison`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeAbs_Poison` `V`

### `RetaliationAttributeDurBonus` (Game.dll, 8)

- `AddDamageToAccumulator` `VC`
- `RetaliationAttributeDurBonus`
- `RetaliationAttributeDurBonus`
- `RetaliationAttributeDurBonus`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurBonus` `V`

### `RetaliationAttributeDurMod_Aether` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Aether`
- `RetaliationAttributeDurMod_Aether`
- `RetaliationAttributeDurMod_Aether`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Aether` `V`

### `RetaliationAttributeDurMod_AttackSpeed` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_AttackSpeed`
- `RetaliationAttributeDurMod_AttackSpeed`
- `RetaliationAttributeDurMod_AttackSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_AttackSpeed` `V`

### `RetaliationAttributeDurMod_Bleeding` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Bleeding`
- `RetaliationAttributeDurMod_Bleeding`
- `RetaliationAttributeDurMod_Bleeding`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Bleeding` `V`

### `RetaliationAttributeDurMod_Chaos` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Chaos`
- `RetaliationAttributeDurMod_Chaos`
- `RetaliationAttributeDurMod_Chaos`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Chaos` `V`

### `RetaliationAttributeDurMod_Cold` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Cold`
- `RetaliationAttributeDurMod_Cold`
- `RetaliationAttributeDurMod_Cold`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Cold` `V`

### `RetaliationAttributeDurMod_DefensiveAbility` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_DefensiveAbility`
- `RetaliationAttributeDurMod_DefensiveAbility`
- `RetaliationAttributeDurMod_DefensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_DefensiveAbility` `V`

### `RetaliationAttributeDurMod_Fire` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Fire`
- `RetaliationAttributeDurMod_Fire`
- `RetaliationAttributeDurMod_Fire`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Fire` `V`

### `RetaliationAttributeDurMod_Life` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Life`
- `RetaliationAttributeDurMod_Life`
- `RetaliationAttributeDurMod_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Life` `V`

### `RetaliationAttributeDurMod_LifeLeach` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_LifeLeach`
- `RetaliationAttributeDurMod_LifeLeach`
- `RetaliationAttributeDurMod_LifeLeach`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_LifeLeach` `V`

### `RetaliationAttributeDurMod_Lightning` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Lightning`
- `RetaliationAttributeDurMod_Lightning`
- `RetaliationAttributeDurMod_Lightning`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Lightning` `V`

### `RetaliationAttributeDurMod_ManaLeach` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_ManaLeach`
- `RetaliationAttributeDurMod_ManaLeach`
- `RetaliationAttributeDurMod_ManaLeach`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_ManaLeach` `V`

### `RetaliationAttributeDurMod_OffensiveAbility` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_OffensiveAbility`
- `RetaliationAttributeDurMod_OffensiveAbility`
- `RetaliationAttributeDurMod_OffensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_OffensiveAbility` `V`

### `RetaliationAttributeDurMod_OffensiveReduction` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_OffensiveReduction`
- `RetaliationAttributeDurMod_OffensiveReduction`
- `RetaliationAttributeDurMod_OffensiveReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_OffensiveReduction` `V`

### `RetaliationAttributeDurMod_Physical` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Physical`
- `RetaliationAttributeDurMod_Physical`
- `RetaliationAttributeDurMod_Physical`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Physical` `V`

### `RetaliationAttributeDurMod_Poison` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_Poison`
- `RetaliationAttributeDurMod_Poison`
- `RetaliationAttributeDurMod_Poison`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_Poison` `V`

### `RetaliationAttributeDurMod_RunSpeed` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_RunSpeed`
- `RetaliationAttributeDurMod_RunSpeed`
- `RetaliationAttributeDurMod_RunSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_RunSpeed` `V`

### `RetaliationAttributeDurMod_SpellCastSpeed` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationModifierTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadModifierTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDurMod_SpellCastSpeed`
- `RetaliationAttributeDurMod_SpellCastSpeed`
- `RetaliationAttributeDurMod_SpellCastSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDurMod_SpellCastSpeed` `V`

### `RetaliationAttributeDur_Aether` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Aether`
- `RetaliationAttributeDur_Aether`
- `RetaliationAttributeDur_Aether`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Aether` `V`

### `RetaliationAttributeDur_AttackSpeed` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_AttackSpeed`
- `RetaliationAttributeDur_AttackSpeed`
- `RetaliationAttributeDur_AttackSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_AttackSpeed` `V`

### `RetaliationAttributeDur_Bleeding` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Bleeding`
- `RetaliationAttributeDur_Bleeding`
- `RetaliationAttributeDur_Bleeding`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Bleeding` `V`

### `RetaliationAttributeDur_Chaos` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Chaos`
- `RetaliationAttributeDur_Chaos`
- `RetaliationAttributeDur_Chaos`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Chaos` `V`

### `RetaliationAttributeDur_Cold` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Cold`
- `RetaliationAttributeDur_Cold`
- `RetaliationAttributeDur_Cold`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Cold` `V`

### `RetaliationAttributeDur_DefensiveAbility` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_DefensiveAbility`
- `RetaliationAttributeDur_DefensiveAbility`
- `RetaliationAttributeDur_DefensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_DefensiveAbility` `V`

### `RetaliationAttributeDur_Fire` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Fire`
- `RetaliationAttributeDur_Fire`
- `RetaliationAttributeDur_Fire`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Fire` `V`

### `RetaliationAttributeDur_Life` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Life`
- `RetaliationAttributeDur_Life`
- `RetaliationAttributeDur_Life`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Life` `V`

### `RetaliationAttributeDur_LifeLeach` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_LifeLeach`
- `RetaliationAttributeDur_LifeLeach`
- `RetaliationAttributeDur_LifeLeach`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_LifeLeach` `V`

### `RetaliationAttributeDur_Lightning` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Lightning`
- `RetaliationAttributeDur_Lightning`
- `RetaliationAttributeDur_Lightning`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Lightning` `V`

### `RetaliationAttributeDur_ManaLeach` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_ManaLeach`
- `RetaliationAttributeDur_ManaLeach`
- `RetaliationAttributeDur_ManaLeach`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_ManaLeach` `V`

### `RetaliationAttributeDur_OffensiveAbility` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_OffensiveAbility`
- `RetaliationAttributeDur_OffensiveAbility`
- `RetaliationAttributeDur_OffensiveAbility`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_OffensiveAbility` `V`

### `RetaliationAttributeDur_OffensiveReduction` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_OffensiveReduction`
- `RetaliationAttributeDur_OffensiveReduction`
- `RetaliationAttributeDur_OffensiveReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_OffensiveReduction` `V`

### `RetaliationAttributeDur_Physical` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Physical`
- `RetaliationAttributeDur_Physical`
- `RetaliationAttributeDur_Physical`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Physical` `V`

### `RetaliationAttributeDur_Poison` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_Poison`
- `RetaliationAttributeDur_Poison`
- `RetaliationAttributeDur_Poison`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_Poison` `V`

### `RetaliationAttributeDur_RunSpeed` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_RunSpeed`
- `RetaliationAttributeDur_RunSpeed`
- `RetaliationAttributeDur_RunSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_RunSpeed` `V`

### `RetaliationAttributeDur_SpellCastSpeed` (Game.dll, 16)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadDurationMaxTag` `VC`
- `GetLoadDurationMinTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeDur_SpellCastSpeed`
- `RetaliationAttributeDur_SpellCastSpeed`
- `RetaliationAttributeDur_SpellCastSpeed`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeDur_SpellCastSpeed` `V`

### `RetaliationAttributeInfluence` (Game.dll, 7)

- `RetaliationAttributeInfluence`
- `RetaliationAttributeInfluence`
- `RetaliationAttributeInfluence`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeInfluence` `V`

### `RetaliationAttributeReflex` (Game.dll, 7)

- `RetaliationAttributeReflex`
- `RetaliationAttributeReflex`
- `RetaliationAttributeReflex`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex` `V`

### `RetaliationAttributeReflex_Freeze` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Freeze`
- `RetaliationAttributeReflex_Freeze`
- `RetaliationAttributeReflex_Freeze`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Freeze` `V`

### `RetaliationAttributeReflex_Knockdown` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Knockdown`
- `RetaliationAttributeReflex_Knockdown`
- `RetaliationAttributeReflex_Knockdown`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Knockdown` `V`

### `RetaliationAttributeReflex_Petrify` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Petrify`
- `RetaliationAttributeReflex_Petrify`
- `RetaliationAttributeReflex_Petrify`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Petrify` `V`

### `RetaliationAttributeReflex_Sleep` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Sleep`
- `RetaliationAttributeReflex_Sleep`
- `RetaliationAttributeReflex_Sleep`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Sleep` `V`

### `RetaliationAttributeReflex_Stun` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Stun`
- `RetaliationAttributeReflex_Stun`
- `RetaliationAttributeReflex_Stun`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Stun` `V`

### `RetaliationAttributeReflex_Trap` (Game.dll, 14)

- `GetDisplayTag` `VC`
- `GetLoadChanceTag` `VC`
- `GetLoadGlobalTag` `VC`
- `GetLoadValueMaxTag` `VC`
- `GetLoadValueMinTag` `VC`
- `GetLoadXorTag` `VC`
- `GetType` `VC`
- `RetaliationAttributeReflex_Trap`
- `RetaliationAttributeReflex_Trap`
- `RetaliationAttributeReflex_Trap`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeReflex_Trap` `V`

### `RetaliationAttributeStore` (Game.dll, 9)

- `ProcessText` `VC`
- `RetaliationAttributeStore`
- `RetaliationAttributeStore`
- `RetaliationAttributeStore`
- `SetGlobalChance`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeStore` `V`

### `RetaliationAttributeStore_Equipment` (Game.dll, 8)

- `Load` `V`
- `RetaliationAttributeStore_Equipment`
- `RetaliationAttributeStore_Equipment`
- `RetaliationAttributeStore_Equipment`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeStore_Equipment` `V`

### `RetaliationAttributeStore_Max` (Game.dll, 8)

- `Load` `V`
- `RetaliationAttributeStore_Max`
- `RetaliationAttributeStore_Max`
- `RetaliationAttributeStore_Max`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeStore_Max` `V`

### `RetaliationAttributeStore_Min` (Game.dll, 8)

- `Load` `V`
- `RetaliationAttributeStore_Min`
- `RetaliationAttributeStore_Min`
- `RetaliationAttributeStore_Min`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeStore_Min` `V`

### `RetaliationAttributeStore_Skill` (Game.dll, 8)

- `Load` `V`
- `RetaliationAttributeStore_Skill`
- `RetaliationAttributeStore_Skill`
- `RetaliationAttributeStore_Skill`
- ``vftable'`
- `operator=`
- `operator=`
- `~RetaliationAttributeStore_Skill` `V`

### `RiftgateSectorData` (Engine.dll, 7)

- `RiftgateSectorData`
- `RiftgateSectorData`
- `RiftgateSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~RiftgateSectorData` `V`

### `RiggedLight` (Engine.dll, 5)

- `RiggedLight`
- `RiggedLight`
- `RiggedLight`
- `operator=`
- `operator=`

### `RigidBodyDescription` (Engine.dll, 8)

- `GetRotationAngles` `S`
- `Init`
- `RigidBodyDescription`
- `RigidBodyDescription`
- `RigidBodyDescription`
- `operator=`
- `operator=`
- `~RigidBodyDescription`

### `Rollup` (Widget.dll, 11)

- `Create`
- `Destroy`
- `ExpandPage`
- `GetDesiredHeight` `C`
- `GetHandle`
- `InsertPage`
- `OnChildEvent` `V`
- `OnDestroy` `V`
- `OnPaint` `V`
- `OnSizeChange` `V`
- `RemovePage`

### `RotateAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `RotateAction`
- `RotateAction`
- `ToString` `VC`
- ``vftable'`
- `~RotateAction` `V`

### `RotateActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `RotateActionPacket`
- `RotateActionPacket`
- `Serialize` `V`
- ``vftable'`
- `operator=`
- `~RotateActionPacket` `V`

### `RouterServices` (Engine.dll, 17)

- `ForceInitialize`
- `GetPublicIP` `C`
- `GetRouterConnectionType`
- `GetRouterStatus`
- `HasBeenInitialized`
- `Initialize`
- `IsMine`
- `LogMappingsToConsole`
- `MapPort`
- `MapRandomPort`
- `RouterServices`
- `RouterServices`
- `ServicesAvailable`
- `ServicesEnabled`
- `UnmapAllGDMappings`
- `UnmapPort`
- `~RouterServices`

### `Rubble` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `Rubble`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Rubble` `V`

### `SaveManager` (Engine.dll, 5)

- `AddJob`
- `DirectRead`
- `DirectWrite`
- `SetEnabled`
- `WaitForCompletion`

### `ScriptEntity` (Game.dll, 22)

- `AddToScene` `V`
- `GetHitBox` `VC`
- `GetHitBox` `VC`
- `GetIntersection` `VC`
- `GetMeshInstance` `VC`
- `GetNumHitBoxes` `VC`
- `GetPhysicsMesh` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsStatic` `VC`
- `Load` `V`
- `OccludesPathing` `VC`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ScriptEntity`
- `ShouldServerSpawn` `VC`
- `UpdateBoundingBox` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~ScriptEntity` `V`

### `ScriptableAction` (Game.dll, 12)

- `Create` `S`
- `GetType` `C`
- `IsReward`
- `Localize` `VC`
- `ModifyValue` `V`
- `OnActivate` `V`
- `OnDeactivate` `V`
- `ScriptableAction`
- `ScriptableAction`
- ``vftable'`
- `operator=`
- `~ScriptableAction` `V`

### `ScriptableActionCollection` (Game.dll, 12)

- `AdjustRewards`
- `Execute` `C`
- `GetActions` `C`
- `HasRewardActions`
- `OnActivate`
- `OnDeactivate`
- `Read`
- `ScriptableActionCollection`
- `ScriptableActionCollection`
- `SpawnAction`
- `operator=`
- `~ScriptableActionCollection`

### `ScriptableAction_BeginQuestTask` (Game.dll, 11)

- `Execute` `V`
- `GetQuestName` `C`
- `GetTaskUid` `C`
- `Read` `V`
- `ScriptableAction_BeginQuestTask`
- `ScriptableAction_BeginQuestTask`
- `ScriptableAction_BeginQuestTask`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_BeginQuestTask` `V`

### `ScriptableAction_CastSkill` (Game.dll, 10)

- `Execute` `V`
- `OnActivate` `V`
- `Read` `V`
- `ScriptableAction_CastSkill`
- `ScriptableAction_CastSkill`
- `ScriptableAction_CastSkill`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_CastSkill` `V`

### `ScriptableAction_GenerateRandomValue` (Game.dll, 10)

- `ClearAllRandomValues` `S`
- `Execute` `V`
- `GetRandomValue` `S`
- `Read` `V`
- `ScriptableAction_GenerateRandomValue`
- `ScriptableAction_GenerateRandomValue`
- ``vftable'`
- `operator=`
- `randomMap` `S`
- `~ScriptableAction_GenerateRandomValue` `V`

### `ScriptableAction_GiveAttribPoint` (Game.dll, 12)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `ModifyValue` `V`
- `Read` `V`
- `ScriptableAction_GiveAttribPoint`
- `ScriptableAction_GiveAttribPoint`
- `ScriptableAction_GiveAttribPoint`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveAttribPoint` `V`

### `ScriptableAction_GiveDevotion` (Game.dll, 11)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `Read` `V`
- `ScriptableAction_GiveDevotion`
- `ScriptableAction_GiveDevotion`
- `ScriptableAction_GiveDevotion`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveDevotion` `V`

### `ScriptableAction_GiveExperience` (Game.dll, 11)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `Read` `V`
- `ScriptableAction_GiveExperience`
- `ScriptableAction_GiveExperience`
- `ScriptableAction_GiveExperience`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveExperience` `V`

### `ScriptableAction_GiveFaction` (Game.dll, 2)

- `GetAmount`
- `GetFactionTag`

### `ScriptableAction_GiveItem` (Game.dll, 17)

- `DeleteInfoItems`
- `DeleteUnusedItems`
- `Execute` `V`
- `GenerateItem` `V`
- `GetCount` `C`
- `GetInfoItem` `C`
- `GetItemIds` `C`
- `GetNumItems` `C`
- `OnActivate` `V`
- `OnDeactivate` `V`
- `Read` `V`
- `ScriptableAction_GiveItem`
- `ScriptableAction_GiveItem`
- ``default constructor closure'`
- ``vftable'`
- `operator=`
- `~ScriptableAction_GiveItem` `V`

### `ScriptableAction_GiveLevel` (Game.dll, 11)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `Read` `V`
- `ScriptableAction_GiveLevel`
- `ScriptableAction_GiveLevel`
- `ScriptableAction_GiveLevel`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveLevel` `V`

### `ScriptableAction_GiveMoney` (Game.dll, 11)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `Read` `V`
- `ScriptableAction_GiveMoney`
- `ScriptableAction_GiveMoney`
- `ScriptableAction_GiveMoney`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveMoney` `V`

### `ScriptableAction_GiveRandomItem` (Game.dll, 8)

- `Execute` `V`
- `GenerateItem` `V`
- `OnActivate` `V`
- `ScriptableAction_GiveRandomItem`
- `ScriptableAction_GiveRandomItem`
- ``vftable'`
- `operator=`
- `~ScriptableAction_GiveRandomItem` `V`

### `ScriptableAction_GiveSkillPoint` (Game.dll, 12)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `ModifyValue` `V`
- `Read` `V`
- `ScriptableAction_GiveSkillPoint`
- `ScriptableAction_GiveSkillPoint`
- `ScriptableAction_GiveSkillPoint`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveSkillPoint` `V`

### `ScriptableAction_GiveTribute` (Game.dll, 11)

- `Execute` `V`
- `GetAmount`
- `Localize` `VC`
- `Read` `V`
- `ScriptableAction_GiveTribute`
- `ScriptableAction_GiveTribute`
- `ScriptableAction_GiveTribute`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_GiveTribute` `V`

### `ScriptableAction_PlayVideo` (Game.dll, 10)

- `Execute` `V`
- `GetVideoPath` `C`
- `Read` `V`
- `ScriptableAction_PlayVideo`
- `ScriptableAction_PlayVideo`
- `ScriptableAction_PlayVideo`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_PlayVideo` `V`

### `ScriptableAction_SetFaction` (Game.dll, 2)

- `GetAmount`
- `GetFactionTag`

### `ScriptableAction_UnlockFaction` (Game.dll, 1)

- `GetFactionTag`

### `ScriptableAction_UnlockTutorial` (Game.dll, 10)

- `Execute` `V`
- `GetPage`
- `Read` `V`
- `ScriptableAction_UnlockTutorial`
- `ScriptableAction_UnlockTutorial`
- `ScriptableAction_UnlockTutorial`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableAction_UnlockTutorial` `V`

### `ScriptableCondition` (Game.dll, 23)

- `AddStateDebugText` `C`
- `Clone` `C`
- `Create` `S`
- `GetComparator` `C`
- `GetComparisonText` `C`
- `GetType` `C`
- `HasChanged`
- `IsDirty` `C`
- `Localize` `VC`
- `OnActivate` `V`
- `OnChanged` `V`
- `OnDeactivate` `V`
- `ReadProperties` `V`
- `Reset` `V`
- `ScriptableCondition`
- `ScriptableCondition`
- `SetComparator`
- `SetDifficultyComparison` `V`
- `StreamState` `V`
- `WriteProperties` `VC`
- ``vftable'`
- `operator=`
- `~ScriptableCondition` `V`

### `ScriptableConditionCollection` (Game.dll, 10)

- `Clone` `C`
- `Evaluate` `C`
- `GetConditions` `C`
- `Read`
- `ScriptableConditionCollection`
- `ScriptableConditionCollection`
- `SetDifficultyComparison`
- `SpawnCondition`
- `operator=`
- `~ScriptableConditionCollection`

### `ScriptableCondition_HasFaction` (Game.dll, 12)

- `Evaluate` `VC`
- `GetDescription`
- `GetFactionRequirement` `C`
- `GetFactionType` `C`
- `Read` `V`
- `ScriptableCondition_HasFaction`
- `ScriptableCondition_HasFaction`
- `ScriptableCondition_HasFaction`
- ``vftable'`
- `operator=`
- `operator=`
- `~ScriptableCondition_HasFaction` `V`

### `SectorData` (Engine.dll, 6)

- `Copy` `V`
- `SectorData`
- `SectorData`
- ``vftable'`
- `operator=`
- `~SectorData` `V`

### `SectorDataManager` (Engine.dll, 50)

- `DeleteSectorData`
- `GetCurrentLayer` `C`
- `GetNumSectorData` `C`
- `GetRenderColorAlpha` `C`
- `GetRenderColorOmega` `C`
- `GetSectorData`
- `GetSectorData` `C`
- `IsValidSectorValue` `C`
- `Load`
- `Load`
- `NewSectorData`
- `ReadAdjustmentData`
- `ReadAmbientData`
- `ReadBloomData`
- `ReadBossData`
- `ReadChallengeData`
- `ReadClimateData`
- `ReadDamageData`
- `ReadDayNightCycleData`
- `ReadFogData`
- `ReadLevelLimitData`
- `ReadNameData`
- `ReadPvpData`
- `ReadRiftgateData`
- `ReadSectorData`
- `ReadViewDistanceData`
- `Save`
- `Save`
- `SectorDataManager`
- `SectorDataManager`
- `SetCurrentLayer`
- `SetSectorData`
- `Unload`
- `WriteAdjustmentData` `C`
- `WriteAmbientData` `C`
- `WriteBloomData` `C`
- `WriteBossData` `C`
- `WriteChallengeData` `C`
- `WriteClimateData` `C`
- `WriteDamageData` `C`
- `WriteDayNightCycleData` `C`
- `WriteFogData` `C`
- `WriteLevelLimitData` `C`
- `WriteNameData` `C`
- `WritePvpData` `C`
- `WriteRiftgateData` `C`
- `WriteSectorData` `C`
- `WriteViewDistanceData` `C`
- `operator=`
- `~SectorDataManager`

### `SectorLayers` (Engine.dll, 17)

- `Destroy`
- `Edit`
- `Fill`
- `GetHeight` `C`
- `GetTargetId`
- `GetUniqueIdForValue` `C`
- `GetWidth` `C`
- `Initialize`
- `IsInitialized` `C`
- `Load`
- `PrimeForEdit`
- `Render` `C`
- `Save` `C`
- `SectorLayers`
- `SetHeightCallbacks`
- `ValidateMapping`
- `~SectorLayers` `V`

### `Serializer` (Engine.dll, 26)

- `Deserialize`
- `FastGetProperty`
- `FastWritePropertyID` `C`
- `FastWriteTerminator` `C`
- `GetProperty`
- `IsReading` `C`
- `Register`
- `Register`
- `Register`
- `Register`
- `Register`
- `Serialize` `C`
- `Serializer`
- `Serializer`
- `VerboseGetProperty`
- `VerboseWritePropertyID` `C`
- `VerboseWriteTerminator` `C`
- `WritePropertyID` `C`
- `WriteTerminator` `C`
- `operator<<`
- `operator<<`
- `operator<<`
- `operator<<`
- `operator<<`
- `operator=`
- `~Serializer`

### `ServerAnnouncePacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ServerAnnouncePacket`
- `ServerAnnouncePacket`
- ``vftable'`
- `operator=`
- `~ServerAnnouncePacket` `V`

### `ServerConnectAckPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ServerConnectAckPacket`
- `ServerConnectAckPacket`
- ``vftable'`
- `operator=`
- `~ServerConnectAckPacket` `V`

### `ServerConnectRejectPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ServerConnectRejectPacket`
- `ServerConnectRejectPacket`
- ``vftable'`
- `operator=`
- `~ServerConnectRejectPacket` `V`

### `ServerConnectRequestPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ServerConnectRequestPacket`
- `ServerConnectRequestPacket`
- ``vftable'`
- `operator=`
- `~ServerConnectRequestPacket` `V`

### `ServerConnectionManager` (Engine.dll, 31)

- `CompleteSteamAuth` `V`
- `ConnectToLANServer` `V`
- `ConnectToLoopback` `V`
- `CreateAddressResolver`
- `CreateNewConnection`
- `DisconnectFromServer` `V`
- `DisconnectHost` `V`
- `DumpHostTable` `V`
- `DumpStats` `V`
- `DumpStatsToString` `V`
- `GetClientTagHasAuthKey` `C`
- `GlobalEnableNetwork` `V`
- `HandleControlSocketPacket`
- `HandlePacket` `V`
- `Initialize` `V`
- `InitializeControlSocket` `V`
- `RemoveEntity` `V`
- `SendPacket` `V`
- `SendPacketExcluding` `V`
- `SendPacketToGroup` `V`
- `SendPacketToHost` `V`
- `SendPacketToServer` `V`
- `ServerConnectionManager`
- `ServerConnectionManager`
- `SetClientTagHasAuthKey`
- `Shutdown` `V`
- `StartInternet` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~ServerConnectionManager` `V`

### `ServerEntityList` (Engine.dll, 13)

- `AddEntityToAllClients` `V`
- `CreateNewClient` `V`
- `CreateNewClient` `V`
- `GetClientFrustumList` `V`
- `HandleCreatedEntity` `V`
- `PadTimeout`
- `ServerEntityList`
- `ServerEntityList`
- `Update` `V`
- `UpdateFrustum` `V`
- ``vftable'`
- `operator=`
- `~ServerEntityList` `V`

### `ServerInfo` (Engine.dll, 6)

- `ServerInfo`
- `ServerInfo`
- `ServerInfo`
- `operator=`
- `operator=`
- `~ServerInfo`

### `ServerNetworkShim` (Engine.dll, 7)

- `SendCharacterAction` `V`
- `SendConfigCommand` `V`
- `ServerNetworkShim`
- `ServerNetworkShim`
- ``vftable'`
- `operator=`
- `~ServerNetworkShim` `V`

### `ServerSpawnCompletePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ServerSpawnCompletePacket`
- `ServerSpawnCompletePacket`
- ``vftable'`
- `operator=`
- `~ServerSpawnCompletePacket` `V`

### `SetCausesAngerConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SetCausesAngerConfigCmd`
- `SetCausesAngerConfigCmd`
- ``vftable'`
- `operator=`
- `~SetCausesAngerConfigCmd` `V`

### `SetCausesAngerConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetCausesAngerConfigCmdPacket`
- `SetCausesAngerConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SetCausesAngerConfigCmdPacket` `V`

### `SetDungeonTimerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetDungeonTimerPacket`
- `SetDungeonTimerPacket`
- ``vftable'`
- `operator=`
- `~SetDungeonTimerPacket` `V`

### `SetFactionConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SetFactionConfigCmd`
- `SetFactionConfigCmd`
- ``vftable'`
- `operator=`
- `~SetFactionConfigCmd` `V`

### `SetFactionConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetFactionConfigCmdPacket`
- `SetFactionConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SetFactionConfigCmdPacket` `V`

### `SetInvincibleConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SetInvincibleConfigCmd`
- `SetInvincibleConfigCmd`
- ``vftable'`
- `operator=`
- `~SetInvincibleConfigCmd` `V`

### `SetInvincibleConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetInvincibleConfigCmdPacket`
- `SetInvincibleConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SetInvincibleConfigCmdPacket` `V`

### `SetMutatorPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetMutatorPacket`
- `SetMutatorPacket`
- ``vftable'`
- `operator=`
- `~SetMutatorPacket` `V`

### `SetPetBonusPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetPetBonusPacket`
- `SetPetBonusPacket`
- ``vftable'`
- `operator=`
- `~SetPetBonusPacket` `V`

### `SetPiece` (Game.dll, 43)

- `ApplyCampPositioning`
- `ApplyGridPositioning`
- `ApplyRingPositioning`
- `ApplyRotation`
- `ApplyStandardPositioning`
- `AttachPreview`
- `CreateObjects`
- `CreateParts`
- `DetachPreview`
- `Generate`
- `GetExactLocation`
- `GetFreeLocation`
- `GetIntersection` `VC`
- `GetNearbyLocation`
- `GetOutlyingLocation`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetVisibility` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `OnMoveInLevel` `V`
- `Pick`
- `PickEntry`
- `PickPart`
- `Place`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetPiece`
- `ShouldSaveState` `VC`
- `Spawn`
- `Spawn`
- `StringToRotation` `C`
- `StringToScheme` `C`
- `StringToTag` `C`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SetPiece` `V`

### `SetSurvivalRestartsPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetSurvivalRestartsPacket`
- `SetSurvivalRestartsPacket`
- ``vftable'`
- `operator=`
- `~SetSurvivalRestartsPacket` `V`

### `SetSurvivalTimerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetSurvivalTimerPacket`
- `SetSurvivalTimerPacket`
- ``vftable'`
- `operator=`
- `~SetSurvivalTimerPacket` `V`

### `SetSurvivalWaveTierPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetSurvivalWaveTierPacket`
- `SetSurvivalWaveTierPacket`
- ``vftable'`
- `operator=`
- `~SetSurvivalWaveTierPacket` `V`

### `SetTargetableConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SetTargetableConfigCmd`
- `SetTargetableConfigCmd`
- ``vftable'`
- `operator=`
- `~SetTargetableConfigCmd` `V`

### `SetTargetableConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SetTargetableConfigCmdPacket`
- `SetTargetableConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SetTargetableConfigCmdPacket` `V`

### `ShowCharacterConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ShowCharacterConfigCmd`
- `ShowCharacterConfigCmd`
- ``vftable'`
- `operator=`
- `~ShowCharacterConfigCmd` `V`

### `ShowCharacterConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ShowCharacterConfigCmdPacket`
- `ShowCharacterConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~ShowCharacterConfigCmdPacket` `V`

### `ShowEntityPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ShowEntityPacket`
- `ShowEntityPacket`
- ``vftable'`
- `operator=`
- `~ShowEntityPacket` `V`

### `ShrineHookPack` (Game.dll, 6)

- `LoadHooks` `V`
- `ShrineHookPack`
- `ShrineHookPack`
- ``vftable'`
- `operator=`
- `~ShrineHookPack` `V`

### `ShrineRewardPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `ShrineRewardPacket`
- `ShrineRewardPacket`
- ``vftable'`
- `operator=`
- `~ShrineRewardPacket` `V`

### `SimulationInformation` (Game.dll, 2)

- `ExportInformation`
- `SetCurrentLevelFile`

### `SinglePlayerBasicInfoPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SinglePlayerBasicInfoPacket`
- `SinglePlayerBasicInfoPacket`
- ``vftable'`
- `operator=`
- `~SinglePlayerBasicInfoPacket` `V`

### `SinglePlayerHeartbeatPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SinglePlayerHeartbeatPacket`
- `SinglePlayerHeartbeatPacket`
- ``vftable'`
- `operator=`
- `~SinglePlayerHeartbeatPacket` `V`

### `Singleton<GAME::AsyncWorker>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::AsyncWorker>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::AsyncWorker>`

### `Singleton<GAME::CPUCoreDetector>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::CPUCoreDetector>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::CPUCoreDetector>`

### `Singleton<GAME::DebugRenderManager>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::DebugRenderManager>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::DebugRenderManager>`

### `Singleton<GAME::EventManager>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::EventManager>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::EventManager>`

### `Singleton<GAME::Jukebox>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::Jukebox>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::Jukebox>`

### `Singleton<GAME::NavManager>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::NavManager>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::NavManager>`

### `Singleton<GAME::ObjectManager>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::ObjectManager>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::ObjectManager>`

### `Singleton<GAME::Quest2Repository>` (Game.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::Quest2Repository>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::Quest2Repository>`

### `Singleton<GAME::RouterServices>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::RouterServices>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::RouterServices>`

### `Singleton<GAME::StyleManager>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::StyleManager>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::StyleManager>`

### `Singleton<GAME::Tracker>` (Engine.dll, 8)

- `Destroy` `S`
- `Get` `S`
- `HasInstantiated` `S`
- `Singleton<GAME::Tracker>`
- `operator=`
- `s_criticalSection` `S`
- `s_instance` `S`
- `~Singleton<GAME::Tracker>`

### `SingletonManager` (Engine.dll, 3)

- `Register`
- `UnRegister`
- `Update`

### `SkeletalPose` (Engine.dll, 21)

- `Add`
- `Blend`
- `CacheCoords` `C`
- `ContainsBone` `C`
- `GetBone` `C`
- `GetBoneCoords` `C`
- `Interpolate` `C`
- `IsEmpty`
- `ReadData`
- `ReadReplicationData`
- `RemoveBoneTranslation`
- `Reset`
- `SetBone`
- `SetBoneCoords`
- `SkeletalPose`
- `SkeletalPose`
- `WriteData` `C`
- `WriteReplicationData`
- `affineIdentity` `S`
- `operator=`
- `~SkeletalPose`

### `Skill` (Game.dll, 422)

- `ActivateOnEnemyDeathModifiers`
- `ActivateOnEnemyDeathSecondarySkills`
- `ActivateSecondarySkills` `V`
- `AddBaseSkill`
- `AddExperience` `V`
- `AddMastery`
- `AddModifier`
- `AddModifierCharAttributes` `C`
- `AddModifierConversionAttributes` `C`
- `AddModifierDefenseAttributes` `C`
- `AddModifierFXChanges` `C`
- `AddModifierOffensiveDamageAttributes` `C`
- `AddModifierOffensiveModifierAttributes` `C`
- `AddModifierPetChanges` `C`
- `AddModifierRacialBonusDamage` `C`
- `AddModifierRacialBonusDefense` `C`
- `AddModifierRetaliationAttributes` `C`
- `AddModifierRetaliationModifierAttributes` `C`
- `AddModifierSkillAttributes` `C`
- `AddPetPassiveOffensiveModifierAttributes` `C`
- `AddProjectileEffects` `VC`
- `AddProjectileModifier` `VC`
- `AddSecondarySkill`
- `AddSkillLevel`
- `AddTimeToLive` `V`
- `AddToItemSecondarySkills`
- `AddToItemSkillModifiers`
- `AddToRefreshDurationModifiers`
- `AddWeightedObjectToList` `C`
- `AllowDualWieldWeapons` `C`
- `ApplyActivatedVisualEffects`
- `ApplyActiveWorldVisualEffects`
- `ApplyBuffOtherEffects`
- `ApplyBuffSelfEffects`
- `ApplyCastVisualEffects` `V`
- `ApplyDisruptionCooldownTime` `V`
- `AutoActivateNow` `V`
- `CacheResources` `V`
- `CalculateAllocatedMemory` `VC`
- `CalculateDPS` `V`
- `CalculateMemoryUsage` `VC`
- `CanInterrupt` `VC`
- `CanModifyChanceToRun` `VC`
- `CanSkillBeQueued` `VC`
- `Cancel` `V`
- `CaptureAnimationRagDollInfo`
- `ClampExperience` `V`
- `ClearHitIteration` `V`
- `CollectCombatParameters` `V`
- `CollectComboMultiplier`
- `CollectLocalConversionAttributes` `VC`
- `CollectLocalOffensiveDamageAttributes` `VC`
- `CollectLocalOffensiveModifierAttributes` `VC`
- `CollectLocalRacialBonusDamage` `VC`
- `CollectLocalRetaliationAttributes` `VC`
- `CollectLocalRetaliationDamagePercent` `VC`
- `CollectLocalRetaliationModifierAttributes` `VC`
- `CollectLocalWeaponDamage` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveConversionAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `CollectRetaliationDamage` `C`
- `CollectRetaliationDamagePctFromModifiers` `VC`
- `CollectWeaponDamage` `C`
- `CollectWeaponDamageFromModifiers` `VC`
- `CollectWeaponParameters` `C`
- `CreateEndBuffOtherFx` `V`
- `CreateEndBuffSelfFx` `V`
- `CreateProjectile` `V`
- `CreateTargetFx` `V`
- `CreateUIAlternateNextPetText` `VC`
- `CreateUIAlternatePetText` `VC`
- `CreateUIModifierText` `VC`
- `CreateUINameText` `VC`
- `CreateUINextModifierText` `VC`
- `CreateUINextParameterText` `VC`
- `CreateUINextPetLimitText` `VC`
- `CreateUINextPetText` `VC`
- `CreateUINextPetTimeToLiveText` `VC`
- `CreateUINextShortSummaryText` `VC`
- `CreateUINextSpecializedText1` `VC`
- `CreateUINextSpecializedText2` `VC`
- `CreateUIParameterText` `VC`
- `CreateUIPetLimitText` `VC`
- `CreateUIPetText` `VC`
- `CreateUIPetTimeToLiveText` `VC`
- `CreateUIShortSummaryText` `VC`
- `CreateUISkillHeading` `VC`
- `CreateUISkillName` `VC`
- `CreateUISkillName` `VC`
- `CreateUISpecializedText1` `VC`
- `CreateUISpecializedText2` `VC`
- `CreateUITrackerText` `V`
- `CreateWarmUpFx` `V`
- `DecrementAugmentedSkillLevel` `V`
- `DecrementCooldownCharge`
- `DecrementSkillLevel` `V`
- `DisableWeaponTrails`
- `DispellSelfBuff` `V`
- `EnableProjectiles`
- `EndCooldown` `V`
- `ExclusiveSkillCheck` `VC`
- `FilterTargetsInLOS` `VC`
- `GetActiveState` `C`
- `GetAffinityBonus` `C`
- `GetAffinityDependencies` `C`
- `GetAllowMove` `VC`
- `GetAugmentedLevel` `C`
- `GetAutoCastControllerName` `C`
- `GetAutoCastSkill` `C`
- `GetBaseDescriptionTag` `C`
- `GetBaseNameText` `C`
- `GetBaseNamesText` `C`
- `GetBaseSkills` `C`
- `GetBuffOtherCharFxPakName` `C`
- `GetBuffSelfCharFxPakName` `C`
- `GetBuffSkillName` `C`
- `GetChanceWeight` `VC`
- `GetChargeCooldown` `C`
- `GetConstellationDependencies` `C`
- `GetConstellationSelfLocked` `C`
- `GetCooldownCharges` `VC`
- `GetCooldownCompletion` `C`
- `GetCooldownRemaining` `C`
- `GetCooldownTime` `C`
- `GetCooldownTotal` `C`
- `GetCoordsFromCallback` `C`
- `GetCurrentLevel` `VC`
- `GetDevotionExperience` `C`
- `GetDevotionLevel` `C`
- `GetDevotionMaxLevel` `C`
- `GetDevotionParent` `C`
- `GetDisplayNameTag` `C`
- `GetExplosionRadiusTag` `VC`
- `GetForceMovement` `VC`
- `GetGamepadRange` `VC`
- `GetHitIteration` `VC`
- `GetItemModifierInfo` `C`
- `GetItemSetName` `C`
- `GetItemSkillEquipLocation` `C`
- `GetItemSkillItemId` `C`
- `GetItemSkillModifiers` `C`
- `GetLightning` `C`
- `GetManaCost` `C`
- `GetManager`
- `GetManager` `C`
- `GetMasteryId` `C`
- `GetMasteryLevel` `C`
- `GetMasteryLevelRequirement` `C`
- `GetMaxCooldownCharges` `VC`
- `GetMaxLevel` `C`
- `GetModifiedSkillId` `C`
- `GetModifierInfo` `C`
- `GetModifiers` `C`
- `GetMoveType` `VC`
- `GetMoveTypeStateString` `VC`
- `GetNumProjectiles` `VC`
- `GetParticleEffect1` `VC`
- `GetParticleEffect2` `VC`
- `GetParticleEffect3` `VC`
- `GetParticleEffectAttach1` `VC`
- `GetParticleEffectAttach2` `VC`
- `GetParticleEffectAttach3` `VC`
- `GetPetAutoCastMasterSkillId` `C`
- `GetPetBonus` `C`
- `GetPetExtents` `C`
- `GetPetLimit` `VC`
- `GetPointInLOS`
- `GetPotionScale` `VC`
- `GetPreloadedPet`
- `GetProjectileFragmentsName` `VC`
- `GetProjectileName` `VC`
- `GetProp1`
- `GetProp2`
- `GetQualifyingDualWeapons` `C`
- `GetQualifyingWeapons` `C`
- `GetRTTIClassInfo` `VC`
- `GetRadiusOverride` `VC`
- `GetRange` `VC`
- `GetRangeProfile` `VC`
- `GetReason` `C`
- `GetRefreshCooldownSkillName` `C`
- `GetRefreshCooldownTrigger` `C`
- `GetRefreshDurationModifierData`
- `GetRefreshDurationSkillName` `C`
- `GetRefreshDurationTrigger` `C`
- `GetRefreshSkillTag` `C`
- `GetRequiredExperience` `C`
- `GetRequiresLOS` `VC`
- `GetResourceName` `C`
- `GetSecondarySkills` `C`
- `GetSkillData` `VC`
- `GetSkillDependancies` `C`
- `GetSkillGroup` `VC`
- `GetSkillLevel` `C`
- `GetSkillModifierData` `VC`
- `GetSkillModifierData` `VC`
- `GetSkillOperation` `C`
- `GetSkillProfile` `VC`
- `GetSkillProjectileModifierData` `VC`
- `GetSkillProjectileModifierData` `VC`
- `GetSkillSecondaryModifierData` `VC`
- `GetSkillSet` `C`
- `GetSkillState` `C`
- `GetSkillType` `C`
- `GetSpawnObject` `VC`
- `GetSpawnObject2` `VC`
- `GetSpawnObject3` `VC`
- `GetSpawnObject4` `VC`
- `GetSpawnObjectTimeToLive` `VC`
- `GetStaticClassInfo` `S`
- `GetSubSkillChildIndex` `C`
- `GetSubSkillParentId` `C`
- `GetSuppressLevelTitles` `C`
- `GetTargetFX` `C`
- `GetTargetRadiusTag` `VC`
- `GetTemplateAutoCast` `C`
- `GetTotalSkillAttribute` `VC`
- `GetTrackableIcon` `VC`
- `GetTrackableType` `VC`
- `GetUltimateLevel` `C`
- `GetUnlockLevel` `VC`
- `GetValidFriendTarget` `C`
- `GetValidMeleeTarget` `C`
- `GetValidRangedTarget` `C`
- `GetValidTarget` `VC`
- `GetWarmUpWasActive` `VC`
- `GetWaveDistanceLimit` `VC`
- `GetWeaponIdsForCallback` `VC`
- `GetWeightedSpawnObject` `C`
- `HandleExclusiveSkill` `V`
- `HasAutocastInDbr` `C`
- `HasAutocastSkill` `C`
- `HasIconDisplay` `VC`
- `HasManager`
- `HasTrackerOverrideTag` `C`
- `IncHitIteration` `V`
- `IncrementAugmentedSkillLevel` `V`
- `IncrementComboCharge` `V`
- `IncrementCooldownCharge`
- `IncrementDevotionLevel` `V`
- `IncrementSkillLevel` `V`
- `InitRagDollData` `VC`
- `Install` `V`
- `InstallPetBonus`
- `IsActive` `VC`
- `IsAugmented` `C`
- `IsAutoToggle` `VC`
- `IsBaseSkillEnabled` `C`
- `IsBranchSkill` `VC`
- `IsBuffActive` `VC`
- `IsChanneled` `VC`
- `IsControllingSkill`
- `IsDefaultSkill` `C`
- `IsGrantedEffect` `C`
- `IsHotBarView` `C`
- `IsItemSkill` `C`
- `IsItemSkillAuto`
- `IsLocked` `C`
- `IsManaAvailable` `C`
- `IsPassive` `VC`
- `IsPetDisplayable` `C`
- `IsPotionSkill` `C`
- `IsPrimary` `C`
- `IsQualifyingWeapons` `C`
- `IsRepeatable` `VC`
- `IsRunning` `VC`
- `IsSecondary` `C`
- `IsSkillA` `C`
- `IsSkillBlackListed` `C`
- `IsSkillEnabled` `VC`
- `IsSkillModifier` `C`
- `IsSkillOnCritActive` `VC`
- `IsSkillOnHitActive` `VC`
- `IsSkillTheMasterySkill` `C`
- `IsStaticSkill` `C`
- `IsTargetInLOS` `VC`
- `IsTargetInLOS` `VC`
- `IsTargetInLOS` `VC`
- `IsTargetInRange` `VC`
- `IsTempTrackable` `VC`
- `IsTrackable` `VC`
- `IsValidTarget` `VC`
- `Load` `V`
- `LoadOriginalSkillResource`
- `LoadResources` `V`
- `LoadSounds`
- `MaxSkillDistance` `V`
- `MoveToPointSkill` `VC`
- `NeedsAttackSlot` `C`
- `NotifyUIOnActivate`
- `NotifyUIOnDeactivate`
- `OnDestroy` `V`
- `OnEnemyDeath` `V`
- `OnHitActivation` `V`
- `OnHitActivationSecondarySkills`
- `OnMoved` `V`
- `OnPathFailed` `VC`
- `OnSkillScript` `V`
- `OnTargetInvalid` `VC`
- `ParticleEffect1` `V`
- `ParticleEffect2` `V`
- `ParticleEffect3` `V`
- `PlayActivateSound` `C`
- `PlayActivatedSounds`
- `PlayCastSound` `C`
- `PlayCastSounds`
- `PlayComboChargeSound` `C`
- `PlayDeactivateSound` `C`
- `PlayHitSound` `VC`
- `PlaySkillSound1` `VC`
- `PlaySkillSound2` `VC`
- `PlaySwipeSound` `VC`
- `PlayUnCastSound` `C`
- `PlayWarmUpSound` `C`
- `PlayWeaponSwipeSound` `V`
- `PreLoad` `V`
- `PreLoadPet` `V`
- `PreLoadResources` `V`
- `PrimaryActivateSecondarySkills`
- `PrimaryStopSecondarySkills`
- `ProjectileLimit` `V`
- `QualifyingHandState` `C`
- `QualifyingWeapon` `C`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `ReceiveAugmentUpdate` `V`
- `ReceiveStateUpdate` `V`
- `RecommendedBuffReflexDuration` `S`
- `RecommendedBuffReflexRate` `S`
- `ReleasePets` `V`
- `ReleaseResources` `V`
- `RemoveFromItemSecondarySkills`
- `RemoveFromItemSkillModifiers`
- `RemoveFromRefreshDurationModifiers`
- `RemovePetBonus`
- `RemoveProjectile` `V`
- `RemoveSelfBuff` `V`
- `ReplaceCooldownTime` `V`
- `ResetComboMultiplier`
- `ResolveValidateTarget`
- `ReverseRotateDirection` `VC`
- `SendActiveStateUpdate`
- `SendAugmentUpdate`
- `SendStateUpdate`
- `SetAffinityBonus`
- `SetAffinityDepedency`
- `SetAsGrantedSkillAuto`
- `SetAsItemSkill`
- `SetAsItemSkillAuto`
- `SetAsPotionSkill`
- `SetAsStaticSkill`
- `SetAsTempSkill`
- `SetAugmentedSkillLevel` `V`
- `SetAutoActivationTag`
- `SetAutoTriggerParam`
- `SetAutocastSkill` `V`
- `SetAvailability` `V`
- `SetChanceOnRun`
- `SetChargeCooldown`
- `SetConstellationDependencies`
- `SetConstellationSelfLocked`
- `SetDevotionLevel`
- `SetDevotionParent`
- `SetDisabled` `V`
- `SetExclusiveItemSkillModifier` `V`
- `SetHotBarView`
- `SetIgnoreMana`
- `SetItemSetName`
- `SetItemSkillItemId`
- `SetLocked`
- `SetManager`
- `SetModifiedSkillId`
- `SetPetAutoCastMasterSkillId`
- `SetPetAutocast` `V`
- `SetSkillLevel` `V`
- `SetSkillOperation`
- `SetSubSkillChildIndex`
- `SetSubSkillParentId`
- `SetUnlockLevel` `V`
- `ShouldFilterCaster` `C`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `ShouldSaveSpawnedPets` `VC`
- `ShouldWpUseParentResources` `C`
- `Skill`
- `Skill`
- `SkillLevelChange` `V`
- `SkillSpawnObject` `V`
- `SpecialActivateSecondarySkills` `V`
- `StartCooldown`
- `StartMove` `V`
- `StartNormalAnimation`
- `StartSpecialAnimation`
- `StopSkill` `V`
- `SubtractManaCost`
- `SubtractSkillLevel`
- `SuppressLevelTitles`
- `TargetFriendInformation` `V`
- `TargetInformation` `V`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `TrackerShowDetail` `C`
- `UnInstall` `V`
- `Update` `V`
- `UpdateCooldownCharges` `V`
- `UpdateFxVisibility`
- `UpdateMaxCooldownCharges` `V`
- `UseDefaultWhenUnavailable` `C`
- `ValidateEnemy` `C`
- `WarmUpEffect` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill` `V`

### `SkillActivated` (Game.dll, 34)

- `ActivateNow` `V`
- `AddProjectileEffects` `VC`
- `ApplyBuffOnTarget` `C`
- `ApplyMeleeDamage` `V`
- `ApplyMeleeDamageInherent` `V`
- `ApplyRangedDamage` `V`
- `CameraShake` `V`
- `CreateProjectile` `V`
- `EndAction` `V`
- `ExecuteMeleeAttack` `V`
- `ExecuteRadiusAttack` `V`
- `ExecuteRangedAttack` `V`
- `GetHitCallbackPoint` `V`
- `GetRTTIClassInfo` `VC`
- `GetRunSpeedAcceleration` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetType` `VC`
- `HitAction` `V`
- `Load` `V`
- `OnHitTarget` `V`
- `RTTI_new` `S`
- `ShouldWarmUpBeforeAttack` `V`
- `SkillActivated`
- `SkillActivated`
- `StartAction` `V`
- `SubSkillEnd` `V`
- `SwipeAction` `V`
- `SwipeOffAction` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillActivated` `V`

### `SkillActivatedBuffOther` (Game.dll, 11)

- `GetValidTarget` `VC`
- `HitAction` `V`
- `SkillActivatedBuffOther`
- `SkillActivatedBuffOther`
- `SkillActivatedBuffOther`
- `StartAction` `V`
- `SwipeAction` `V`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillActivatedBuffOther` `V`

### `SkillActivatedBuffSelf` (Game.dll, 23)

- `ApplyActiveCosts`
- `ApplyCost`
- `AutoActivateNow` `V`
- `CalculateDPS` `V`
- `DispellSelfBuff` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTotalSkillAttribute` `VC`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `IsAutoToggle` `VC`
- `LoadResources` `V`
- `RTTI_new` `S`
- `SkillActivatedBuffSelf`
- `SkillActivatedBuffSelf`
- `SkillActivatedBuffSelf`
- `StartAction` `V`
- `SwipeAction` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~SkillActivatedBuffSelf` `V`

### `SkillActivatedSpell` (Game.dll, 18)

- `CalculateDPS` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `PreLoad` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillActivatedSpell`
- `SkillActivatedSpell`
- `StartAction` `V`
- `SwipeAction` `V`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillActivatedSpell` `V`

### `SkillActivatedWeapon` (Game.dll, 22)

- `CalculateDPS` `V`
- `EndAction` `V`
- `GetRTTIClassInfo` `VC`
- `GetRunSpeedAcceleration` `VC`
- `GetSkillGroup` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `RTTI_new` `S`
- `SkillActivatedWeapon`
- `SkillActivatedWeapon`
- `SkillActivatedWeapon`
- `StartAction` `V`
- `SwipeAction` `V`
- `SwipeOffAction` `V`
- `TargetInformation` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~SkillActivatedWeapon` `V`

### `SkillActivatedWeaponPool` (Game.dll, 32)

- `CalculateDPS` `V`
- `CanSkillBeQueued` `VC`
- `EndAction` `V`
- `GetParticleEffect1` `VC`
- `GetParticleEffect2` `VC`
- `GetParticleEffect3` `VC`
- `GetParticleEffectAttach1` `VC`
- `GetParticleEffectAttach2` `VC`
- `GetParticleEffectAttach3` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSkillGroup` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `PlaySkillSound1` `VC`
- `PlaySkillSound2` `VC`
- `RTTI_new` `S`
- `SkillActivatedWeaponPool`
- `SkillActivatedWeaponPool`
- `SkillActivatedWeaponPool`
- `StartAction` `V`
- `SwipeAction` `V`
- `SwipeOffAction` `V`
- `TargetInformation` `V`
- `Update` `V`
- `WPPostAttackCallback` `V`
- `WPPreAttackCallback` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~SkillActivatedWeaponPool` `V`

### `SkillActivated_Suicide` (Game.dll, 13)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HitAction` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SkillActivated_Suicide`
- `SkillActivated_Suicide`
- `StartAction` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillActivated_Suicide` `V`

### `SkillActiveState` (Game.dll, 7)

- `RestoreState`
- `SaveState` `C`
- `SkillActiveState`
- `SkillActiveState`
- `StreamProperties`
- `operator=`
- `~SkillActiveState`

### `SkillActiveStateUpdateConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SkillActiveStateUpdateConfigCmd`
- `SkillActiveStateUpdateConfigCmd`
- ``vftable'`
- `operator=`
- `~SkillActiveStateUpdateConfigCmd` `V`

### `SkillActiveStateUpdateConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SkillActiveStateUpdateConfigCmdPacket`
- `SkillActiveStateUpdateConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SkillActiveStateUpdateConfigCmdPacket` `V`

### `SkillAttribute` (Game.dll, 40)

- `AddJitter` `V`
- `AddMaxJitter` `V`
- `AddMinJitter` `V`
- `CreateNextText` `VC`
- `CreateRangeNumbers` `VC`
- `CreateRangeText` `VC`
- `CreateText` `VC`
- `GetChance` `C`
- `GetModifierChance` `C`
- `GetModifierValue` `C`
- `GetPrefixChance` `C`
- `GetPrefixValue` `C`
- `GetRangeTag` `C`
- `GetSuffixChance` `C`
- `GetSuffixValue` `C`
- `GetTotalValue` `C`
- `GetValue` `C`
- `IsNotEmpty` `C`
- `Jitter`
- `LoadBaseTable` `V`
- `LoadModifierTable` `V`
- `LoadModifierTableMax` `V`
- `LoadModifierTableMin` `V`
- `LoadPrefixTable` `V`
- `LoadPrefixTableMax` `V`
- `LoadPrefixTableMin` `V`
- `LoadSuffixTable` `V`
- `LoadSuffixTableMax` `V`
- `LoadSuffixTableMin` `V`
- `MaxJitter`
- `MergeAttribute`
- `MinJitter`
- `Scale`
- `ScaleAttribute` `V`
- `SetAttributeToLevel`
- `SkillAttribute`
- `SkillAttribute`
- ``vftable'`
- `operator=`
- `~SkillAttribute` `V`

### `SkillAttributeAccumulator` (Game.dll, 13)

- `AddChanceModifier` `V`
- `AddChanceValue` `V`
- `AddModifier` `V`
- `AddValue` `V`
- `Clear`
- `GetRandomGen`
- `GetValue` `C`
- `SetSeed`
- `SkillAttributeAccumulator`
- `SkillAttributeAccumulator`
- ``vftable'`
- `operator=`
- `~SkillAttributeAccumulator` `V`

### `SkillAttributeMod` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `GetCostInfo` `VC`
- `SkillAttributeMod`
- `SkillAttributeMod`
- `SkillAttributeMod`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeMod` `V`

### `SkillAttributeMod_CooldownReduction` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeMod_CooldownReduction`
- `SkillAttributeMod_CooldownReduction`
- `SkillAttributeMod_CooldownReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeMod_CooldownReduction` `V`

### `SkillAttributeMod_ManaCostReduction` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeMod_ManaCostReduction`
- `SkillAttributeMod_ManaCostReduction`
- `SkillAttributeMod_ManaCostReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeMod_ManaCostReduction` `V`

### `SkillAttributeStore` (Game.dll, 22)

- `AddJitter` `V`
- `AddMaxJitter` `V`
- `AddMinJitter` `V`
- `AddToAccumulator` `VC`
- `AddToStore` `V`
- `Clear`
- `CreateNextText` `VC`
- `CreateText` `VC`
- `GetAttributes`
- `GetAttributes` `C`
- `GetCostInfo` `C`
- `GetRandomGen`
- `IsTypePresent` `C`
- `MergeStore`
- `MergeStoreAtLevel`
- `ScaleAttributes` `V`
- `SetRandomGen`
- `SkillAttributeStore`
- `SkillAttributeStore`
- ``vftable'`
- `operator=`
- `~SkillAttributeStore` `V`

### `SkillAttributeStore_Equipment` (Game.dll, 8)

- `Load` `V`
- `SkillAttributeStore_Equipment`
- `SkillAttributeStore_Equipment`
- `SkillAttributeStore_Equipment`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeStore_Equipment` `V`

### `SkillAttributeStore_Max` (Game.dll, 8)

- `Load` `V`
- `SkillAttributeStore_Max`
- `SkillAttributeStore_Max`
- `SkillAttributeStore_Max`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeStore_Max` `V`

### `SkillAttributeStore_Min` (Game.dll, 8)

- `Load` `V`
- `SkillAttributeStore_Min`
- `SkillAttributeStore_Min`
- `SkillAttributeStore_Min`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeStore_Min` `V`

### `SkillAttributeStore_Skill` (Game.dll, 8)

- `Load` `V`
- `SkillAttributeStore_Skill`
- `SkillAttributeStore_Skill`
- `SkillAttributeStore_Skill`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeStore_Skill` `V`

### `SkillAttributeVal` (Game.dll, 9)

- `AddToAccumulator` `VC`
- `GetCostInfo` `VC`
- `SkillAttributeVal`
- `SkillAttributeVal`
- `SkillAttributeVal`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeVal` `V`

### `SkillAttributeVal_ComboChargeSpendReduction` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeVal_ComboChargeSpendReduction`
- `SkillAttributeVal_ComboChargeSpendReduction`
- `SkillAttributeVal_ComboChargeSpendReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeVal_ComboChargeSpendReduction` `V`

### `SkillAttributeVal_CooldownReduction` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeVal_CooldownReduction`
- `SkillAttributeVal_CooldownReduction`
- `SkillAttributeVal_CooldownReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeVal_CooldownReduction` `V`

### `SkillAttributeVal_ManaCostReduction` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeVal_ManaCostReduction`
- `SkillAttributeVal_ManaCostReduction`
- `SkillAttributeVal_ManaCostReduction`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeVal_ManaCostReduction` `V`

### `SkillAttributeVal_ProjectileSpeedModifier` (Game.dll, 8)

- `GetType` `VC`
- `SkillAttributeVal_ProjectileSpeedModifier`
- `SkillAttributeVal_ProjectileSpeedModifier`
- `SkillAttributeVal_ProjectileSpeedModifier`
- ``vftable'`
- `operator=`
- `operator=`
- `~SkillAttributeVal_ProjectileSpeedModifier` `V`

### `SkillAugmentUpdateConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SkillAugmentUpdateConfigCmd`
- `SkillAugmentUpdateConfigCmd`
- ``vftable'`
- `operator=`
- `~SkillAugmentUpdateConfigCmd` `V`

### `SkillAugmentUpdateConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SkillAugmentUpdateConfigCmdPacket`
- `SkillAugmentUpdateConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SkillAugmentUpdateConfigCmdPacket` `V`

### `SkillAutoCastController` (Game.dll, 22)

- `AppraiseAndExecute`
- `AttackingEnemy`
- `CastingBuff`
- `CastingDebuf`
- `ClearCooldownBlock`
- `EnemyDeath`
- `GetGrantingSkillId` `C`
- `GetParams` `S`
- `GetRefreshTag` `S`
- `GetSkillId` `C`
- `HasDied`
- `HitByEnemy`
- `IsCooldownRefresh` `VC`
- `IsDurationRefresh` `VC`
- `IsGrantingSkill`
- `ModifyChanceToRun`
- `SkillAutoCastController`
- `SkillAutoCastController`
- `Update`
- ``vftable'`
- `operator=`
- `~SkillAutoCastController` `V`

### `SkillBuff` (Game.dll, 50)

- `AddTimeToLive` `V`
- `BonusTime` `S`
- `CanModifyChanceToRun` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveConversionAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `DispelBuff` `V`
- `DispelDeBuff` `V`
- `DispelPassiveChargedBuff` `V`
- `DispelPassiveEndlessBuff` `V`
- `GetCasterId` `C`
- `GetParentSkillId` `C`
- `GetPotionScale` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HandleExclusiveSkill` `V`
- `InitializeBuff` `V`
- `Install` `V`
- `IsAura` `C`
- `IsAutoToggle` `VC`
- `IsTempTrackable` `VC`
- `LoadResources` `V`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SetAsAura`
- `SetCasterId`
- `SetParentSkillId`
- `SetSeed`
- `SetTimeToLive` `V`
- `ShouldRemoveOnDeath` `VC`
- `SkillBuff`
- `SkillBuff`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `UnInstall` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff` `V`

### `SkillBuff_BuffImmobilize` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillBuff_BuffImmobilize`
- `SkillBuff_BuffImmobilize`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_BuffImmobilize` `V`

### `SkillBuff_Contageous` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Install` `V`
- `RTTI_new` `S`
- `ShouldRemoveOnDeath` `VC`
- `SkillBuff_Contageous`
- `SkillBuff_Contageous`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_Contageous` `V`

### `SkillBuff_Debuf` (Game.dll, 14)

- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `DispelDeBuff` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillBuff_Debuf`
- `SkillBuff_Debuf`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_Debuf` `V`

### `SkillBuff_DebufFreeze` (Game.dll, 11)

- `ApplyBuffDamage` `V`
- `GetRTTIClassInfo` `VC`
- `GetResistance` `V`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillBuff_DebufFreeze`
- `SkillBuff_DebufFreeze`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_DebufFreeze` `V`

### `SkillBuff_DebufProjectile` (Game.dll, 13)

- `GetNumProjectiles` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadResources` `V`
- `ProjectileLimit` `V`
- `RTTI_new` `S`
- `SkillBuff_DebufProjectile`
- `SkillBuff_DebufProjectile`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_DebufProjectile` `V`

### `SkillBuff_DebufRadius` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillBuff_DebufRadius`
- `SkillBuff_DebufRadius`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_DebufRadius` `V`

### `SkillBuff_DebufTrap` (Game.dll, 15)

- `ApplyBuffDamage` `V`
- `GetRTTIClassInfo` `VC`
- `GetResistance` `V`
- `GetStaticClassInfo` `S`
- `Install` `V`
- `ModifyDuration` `V`
- `RTTI_new` `S`
- `SetTimeToLive` `V`
- `SkillBuff_DebufTrap`
- `SkillBuff_DebufTrap`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillBuff_DebufTrap` `V`

### `SkillBuff_DispelMagic` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Install` `V`
- `Load` `V`
- `RTTI_new` `S`
- `SkillBuff_DispelMagic`
- `SkillBuff_DispelMagic`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff_DispelMagic` `V`

### `SkillBuff_Passive` (Game.dll, 12)

- `DispelBuff` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `RTTI_new` `S`
- `SkillBuff_Passive`
- `SkillBuff_Passive`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff_Passive` `V`

### `SkillBuff_PassiveCharged` (Game.dll, 14)

- `DispelPassiveChargedBuff` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `RTTI_new` `S`
- `SkillBuff_PassiveCharged`
- `SkillBuff_PassiveCharged`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff_PassiveCharged` `V`

### `SkillBuff_PassiveEndless` (Game.dll, 11)

- `DispelPassiveEndlessBuff` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillBuff_PassiveEndless`
- `SkillBuff_PassiveEndless`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff_PassiveEndless` `V`

### `SkillBuff_PassiveShield` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitializeBuff` `V`
- `ModifyDamage` `V`
- `RTTI_new` `S`
- `SkillBuff_PassiveShield`
- `SkillBuff_PassiveShield`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillBuff_PassiveShield` `V`

### `SkillChanneled` (Game.dll, 19)

- `ActivateNow` `V`
- `ForceEnd`
- `GetManaCostPeriod` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `IsChanneled` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `SkillChanneled`
- `SkillChanneled`
- `SkillChanneled`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~SkillChanneled` `V`

### `SkillComboChargeUpdateConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SkillComboChargeUpdateConfigCmd`
- `SkillComboChargeUpdateConfigCmd`
- ``vftable'`
- `operator=`
- `~SkillComboChargeUpdateConfigCmd` `V`

### `SkillComboChargeUpdateConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SkillComboChargeUpdateConfigCmdPacket`
- `SkillComboChargeUpdateConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SkillComboChargeUpdateConfigCmdPacket` `V`

### `SkillLocation` (Game.dll, 14)

- `DebugRender` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `RTTI_new` `S`
- `SkillLocation`
- `UpdateSelf` `V`
- `_DrawEditorArrow` `C`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SkillLocation` `V`

### `SkillManager` (Game.dll, 212)

- `AddCharFxPak` `V`
- `AddExperience`
- `AddItemSkillModifiers`
- `AddItemSkills`
- `AddMasteryAllowedIncrementLevel`
- `AddSubSkills`
- `AddToActiveList` `V`
- `AddToScrollSkillList`
- `AddToUISkillList`
- `AddWeaponEnchantment` `V`
- `AllowDualWieldWeapons`
- `ApplyCooldownDamage`
- `CalculateAllocatedMemory` `C`
- `CalculateOffensiveAbility` `VC`
- `CancelSkillAction`
- `ClearAllowedSkillTypes`
- `ClearCurrentCharFxPaks`
- `CollectAvailableConversionAttributes` `C`
- `CollectAvailableOffensiveDamageAttributes` `C`
- `CollectAvailableOffensiveDamageAttributes` `VC`
- `CollectAvailableOffensiveModifierAttributes` `VC`
- `CollectAvailableRetaliationAttributes` `C`
- `CollectAvailableRetaliationModifierAttributes` `C`
- `CollectAvailableSkillAttributes` `C`
- `CollectWeaponTypes`
- `ContributeRacialBonusDamage` `C`
- `ContributeRacialBonusDefense` `C`
- `CreateAndLoadSkill`
- `CreateFxPak` `V`
- `CreateProjectile`
- `CreateRemoveSkillBuff`
- `CreateSkillBuff`
- `DebugIncrementMasteriesAllowed`
- `DeleteProjectile`
- `DispelEndlessShrineBuffs`
- `DispelEndlessSoulBuffs`
- `DispelSkillBuffs`
- `DispelSkillDeBuffs`
- `EndCooldown` `V`
- `FastBuffLevelUpdate`
- `FindCachedItemSkillIdbyItem` `C`
- `FindItemSkillId` `C`
- `FindItemSkillIdByItem` `C`
- `FindItemSkillIdByItemId` `C`
- `FindSkillId` `C`
- `ForceSpeedUpdate` `V`
- `GetActiveLifeCost` `C`
- `GetActiveManaCost` `C`
- `GetActiveSkillList` `C`
- `GetArmorId` `C`
- `GetAttackSpeed` `VC`
- `GetBuffSkill` `C`
- `GetCharAttributes` `C`
- `GetComboCharge` `C`
- `GetComboChargeDuration` `C`
- `GetConversionAttributes` `C`
- `GetCurrentDevotionReclamationCost` `C`
- `GetCurrentMana` `VC`
- `GetCurrentSkillReclamationCost` `C`
- `GetCurrentSkillSet` `C`
- `GetDefaultSkillId` `VC`
- `GetDefaultSkillIds` `C`
- `GetDefenseAttributes` `C`
- `GetDevotionReclamationAetherCost` `C`
- `GetExclusiveSkill` `VC`
- `GetHandState` `VC`
- `GetItemSkillCache` `C`
- `GetItemSkillList` `C`
- `GetItemSkillReplica` `C`
- `GetLeftHandWeapon` `VC`
- `GetMoveType` `C`
- `GetNumDevotionPointsSpent` `C`
- `GetNumMasteryPoints` `C`
- `GetNumOfNonMasterySkillsWithPoints` `C`
- `GetNumRegularSkillPoints` `C`
- `GetOffensiveDamageAttributes` `C`
- `GetOffensiveModifierAttributes` `C`
- `GetParent` `VC`
- `GetReflectLinkDamageReduction` `C`
- `GetRetaliationAttributes` `C`
- `GetRetaliationModifierAttributes` `C`
- `GetRightHandWeapon` `VC`
- `GetSkillAttributes` `C`
- `GetSkillBuffReplica` `C`
- `GetSkillIdFromReference` `C`
- `GetSkillList` `C`
- `GetSkillMasteries` `C`
- `GetSkillMasteriesActive` `C`
- `GetSkillMasteriesAllowed` `C`
- `GetSkillMasteryIds` `C`
- `GetSkillReferenceNumber` `C`
- `GetSkillReplica` `C`
- `GetSkillServices` `V`
- `GetSkillServicesReplica`
- `GetSpellCastSpeed` `VC`
- `GetSubSkillsList` `C`
- `GetTargetType` `C`
- `GetTotalSkillAttribute` `VC`
- `GetUIItemSkillList` `C`
- `GetUISkillList` `C`
- `GetUITempSkillList` `C`
- `GetWeaponRangeProfile` `VC`
- `GetWeaponTypes` `VC`
- `HandleSkillAnimationCallback`
- `HasActiveDebuffs` `C`
- `ImDead`
- `IncrementComboCharge` `V`
- `IncrementSkill`
- `Initialize`
- `IsDefaultSkill` `C`
- `IsDispelBuffActive`
- `IsGlobalSkillTypeAndAllowed` `C`
- `IsItemSkillActive` `VC`
- `IsLeftHandWeapon` `VC`
- `IsMonsterSkill` `VC`
- `IsOnActiveList` `VC`
- `IsPurityActive` `C`
- `IsRangedWeapon` `VC`
- `IsRunningSkill` `C`
- `IsSkillBuffActive` `C`
- `IsSkillToggled` `C`
- `IsSkillValidForUse` `C`
- `IsSpeedUpdateNeeded`
- `Load` `V`
- `LoadDefaultPlayerSkills`
- `LoadDefaultSkills`
- `LoadSkills`
- `LoadSkills`
- `Moved`
- `OnCriticalAttack`
- `OnDestroy` `V`
- `OnEnemyDeath`
- `ParentIsMonster`
- `PreLoad` `V`
- `ReadProperties`
- `RecalculateSkills`
- `ReflectLinkDamage` `C`
- `RefreshCooldown` `V`
- `RegisterAutoCastSkill` `V`
- `RegisterAutoCastSkills` `V`
- `RegisterRefreshSkillCooldown` `V`
- `RegisterRefreshSkillDuration` `V`
- `RemoveAllSkillAugment`
- `RemoveCharFxPak` `V`
- `RemoveFromActiveList` `V`
- `RemoveFromUISkillList`
- `RemoveItemSkillModifiers`
- `RemoveItemSkills`
- `RemoveMasteryAugment`
- `RemoveSkillBuffs`
- `RemoveSkillLevelAugment`
- `RemoveSubSkills`
- `RemoveWeaponEnchantment` `V`
- `ResetNonStaticSkills`
- `ResolveEquationVariable` `VC`
- `ResolveItemSkillId` `C`
- `ResolveSubSkillId` `C`
- `RestoreSkillCooldown`
- `RetrieveCachedItemSkill`
- `RetrieveCachedItemSkill`
- `SelectWeaponPoolSkill` `VC`
- `SendSkillAugmentUpdate`
- `SetAllSkillAugment`
- `SetAllowedSkillTypes`
- `SetAsControllingManager`
- `SetAsItemSkills`
- `SetAsStaticSkills`
- `SetComboChargeEffect`
- `SetCurrentSkillSet`
- `SetExclusiveSkill` `V`
- `SetHideSkillsFromUI`
- `SetItemSkillReplica`
- `SetMasteryAugment`
- `SetSkillBuffReplica`
- `SetSkillLevelAugment`
- `SetSkillReplica`
- `SetSkillServicesReplica`
- `SetSpeedUpdateNeeded` `V`
- `SkillActiveStateUpdateCommand`
- `SkillAugmentUpdateCommand`
- `SkillComboChargeUpdateCommand`
- `SkillManager`
- `SkillManager`
- `SkillSpawnObject`
- `SkillStateUpdateCommand`
- `SkillTargetResult`
- `SkillWarmUp`
- `SkillWarmUpBeforeAttack`
- `StartSkill`
- `StopCurrentSkill`
- `StoreCachedItemSkill`
- `StoreSkillCooldown`
- `StreamProperties`
- `SubtractLife` `V`
- `SubtractMana` `V`
- `UnderAttack`
- `UnregisterAutoCastSkill` `V`
- `UnregisterRefreshSkillCooldown` `V`
- `UnregisterRefreshSkillDuration` `V`
- `Update`
- `UpdateMasteriesAllowed`
- `UpdatePets`
- `UpdateSubSkillLevels`
- `UseDevotionReclamationPoints`
- `UseReclamationPoints`
- `WriteProperties` `C`
- ``vftable'`
- ``vftable'`
- `kAlternateEquipmentFlag` `S`
- `kAlternateEquipmentMask` `S`
- `kNumSkills` `S`
- `~SkillManager` `V`

### `SkillManagerBase` (Game.dll, 5)

- `SkillManagerBase`
- `SkillManagerBase`
- ``vftable'`
- `operator=`
- `~SkillManagerBase` `V`

### `SkillManagerLite` (Game.dll, 50)

- `AddCharFxPak` `V`
- `AddToActiveList` `V`
- `AddWeaponEnchantment` `V`
- `CalculateOffensiveAbility` `VC`
- `CollectAvailableOffensiveDamageAttributes` `VC`
- `CollectAvailableOffensiveModifierAttributes` `VC`
- `CreateFxPak` `V`
- `EndCooldown` `V`
- `ForceSpeedUpdate` `V`
- `GetAttackSpeed` `VC`
- `GetCurrentMana` `VC`
- `GetDefaultSkillId` `VC`
- `GetExclusiveSkill` `VC`
- `GetHandState` `VC`
- `GetLeftHandWeapon` `VC`
- `GetParent` `VC`
- `GetRightHandWeapon` `VC`
- `GetSkillServices` `V`
- `GetSpellCastSpeed` `VC`
- `GetTotalSkillAttribute` `VC`
- `GetWeaponRangeProfile` `VC`
- `GetWeaponTypes` `VC`
- `IncrementComboCharge` `V`
- `IsItemSkillActive` `VC`
- `IsLeftHandWeapon` `VC`
- `IsMonsterSkill` `VC`
- `IsOnActiveList` `VC`
- `IsRangedWeapon` `VC`
- `RefreshCooldown` `V`
- `RegisterAutoCastSkill` `V`
- `RegisterAutoCastSkills` `V`
- `RegisterRefreshSkillCooldown` `V`
- `RegisterRefreshSkillDuration` `V`
- `RemoveCharFxPak` `V`
- `RemoveFromActiveList` `V`
- `RemoveWeaponEnchantment` `V`
- `SelectWeaponPoolSkill` `VC`
- `SetExclusiveSkill` `V`
- `SetSpeedUpdateNeeded` `V`
- `SkillManagerLite`
- `SkillManagerLite`
- `SubtractLife` `V`
- `SubtractMana` `V`
- `UnregisterAutoCastSkill` `V`
- `UnregisterRefreshSkillCooldown` `V`
- `UnregisterRefreshSkillDuration` `V`
- `Update`
- ``vftable'`
- `operator=`
- `~SkillManagerLite` `V`

### `SkillOnDeath` (Game.dll, 12)

- `ActivateOnDeathNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillOnDeath`
- `SkillOnDeath`
- `SkillOnDeath`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~SkillOnDeath` `V`

### `SkillProfile` (Game.dll, 160)

- `AllowsWarmUp` `C`
- `FilterCaster` `C`
- `GetActivateSoundName` `C`
- `GetActivationChance` `C`
- `GetActiveAuraName` `C`
- `GetActorScale` `C`
- `GetActorScaleTime` `C`
- `GetAlternateSourceLocationId` `C`
- `GetBaseDescriptionTag` `C`
- `GetBonusEffectName` `C`
- `GetCacheableResources` `C`
- `GetCastAuraName` `C`
- `GetCastSoundName` `C`
- `GetChanceWeight` `C`
- `GetCharAttrStore` `C`
- `GetCharAttrStorePenalty` `C`
- `GetCharAttrStorePenaltyReduction` `C`
- `GetCharAttributes` `C`
- `GetCharAttributesPenalty` `C`
- `GetCharAttributesPenaltyReduction` `C`
- `GetCharBuffFxType` `C`
- `GetCharFxOtherName` `C`
- `GetCharFxSelfName` `C`
- `GetChargeAuraName` `C`
- `GetChargeDuration` `C`
- `GetChargeMultipliers` `C`
- `GetChargeTime` `C`
- `GetComboChargeSoundName` `C`
- `GetConnectorsOff` `C`
- `GetConnectorsOn` `C`
- `GetContagionInterval` `C`
- `GetContagionLimit` `C`
- `GetContagionMaxSpread` `C`
- `GetContagionRadius` `C`
- `GetConversionAttrStore` `C`
- `GetConversionAttributes` `C`
- `GetDamageAbsorption` `C`
- `GetDamageAbsorptionReflectPercent` `C`
- `GetDamageAttrStore` `C`
- `GetDeactivateSoundName` `C`
- `GetDefenseAttrStore` `C`
- `GetDefenseAttributes` `C`
- `GetDevotionMaxLevel` `C`
- `GetDisplayNameTag` `C`
- `GetDownAlternateBitmapName` `C`
- `GetDownBitmapName` `C`
- `GetDownRoundBitmapName` `C`
- `GetEndBuffOtherFxPakName` `C`
- `GetEndBuffSelfFxPakName` `C`
- `GetHitSoundName` `C`
- `GetLifeMonitorPercent` `C`
- `GetLightningName` `C`
- `GetMasteryLevelRequirement` `C`
- `GetMaxChargeLevel` `C`
- `GetMaxLevel` `C`
- `GetMaxMoveRatio` `C`
- `GetMaximumNumberOfProjectiles` `C`
- `GetNotDispelSetting` `C`
- `GetNumberOfProjectiles` `C`
- `GetNumberOfSpawnObjects` `C`
- `GetOffensiveDamageAttributes` `C`
- `GetOffensiveModifierAttributes` `C`
- `GetParticleEffectAttachPoint1` `C`
- `GetParticleEffectAttachPoint2` `C`
- `GetParticleEffectAttachPoint3` `C`
- `GetParticleEffectName1` `C`
- `GetParticleEffectName2` `C`
- `GetParticleEffectName3` `C`
- `GetPetBonusScaling` `C`
- `GetPetDisplayable` `C`
- `GetPetPeriod` `C`
- `GetPlaySpecialAttackSound1` `C`
- `GetPlaySpecialAttackSound2` `C`
- `GetPlaySpecialAttackSound3` `C`
- `GetPlaySpecialAttackSound4` `C`
- `GetProjModImpactFxPakName` `C`
- `GetProjectileAngle` `C`
- `GetProjectileHitTimeToLive` `C`
- `GetProjectileMissTimeToLive` `C`
- `GetProjectileName` `C`
- `GetProjectileTargetGroundOnly` `C`
- `GetPropName1` `C`
- `GetPropName2` `C`
- `GetPuppetName` `C`
- `GetQualifyingDamageTags` `C`
- `GetQualifyingDualWeapons` `C`
- `GetQualifyingNoWeapons` `C`
- `GetQualifyingWeapons` `C`
- `GetRacialBonus` `C`
- `GetRacialBonusDamage` `C`
- `GetRacialBonusDefense` `C`
- `GetRacialProfile` `C`
- `GetRadiusEffectName` `C`
- `GetRadiusMagicName` `C`
- `GetRadiusTime` `C`
- `GetRagDollAmplification` `C`
- `GetRagDollDirection` `C`
- `GetRagDollEffect` `C`
- `GetRagDollElevation` `C`
- `GetRagDollPush` `C`
- `GetRangedChargeTime` `C`
- `GetRequiredExperience` `C`
- `GetRetaliationAttrStore` `C`
- `GetRetaliationAttributes` `C`
- `GetRetaliationDamagePct` `C`
- `GetRetaliationModifierAttributes` `C`
- `GetSkillAttrStore` `C`
- `GetSkillAttributes` `C`
- `GetSkillDependancies` `C`
- `GetSkillEnhancement` `C`
- `GetSkillModifierData` `C`
- `GetSkillProjectileModifierData` `C`
- `GetSkillSound1Name` `C`
- `GetSkillSound2Name` `C`
- `GetSkillTier` `C`
- `GetSpawnObject1` `C`
- `GetSpawnObject2` `C`
- `GetSpawnObject3` `C`
- `GetSpawnObject4` `C`
- `GetSpawnObjectsDistanceIncrement` `C`
- `GetSpawnObjectsDistanceInnerCircle` `C`
- `GetSpawnObjectsNumberOfRings` `C`
- `GetSpawnObjectsSpacingAngle` `C`
- `GetSpawnWeight1` `C`
- `GetSpawnWeight2` `C`
- `GetSpawnWeight3` `C`
- `GetSpawnWeight4` `C`
- `GetSpecialAnimationName` `C`
- `GetSpellAuraDuration` `C`
- `GetSpellAuraName` `C`
- `GetSwipeSoundName` `C`
- `GetTargetFxPakName` `C`
- `GetTargetingMode` `C`
- `GetUltimateLevel` `C`
- `GetUnCastSoundName` `C`
- `GetUpAlternateBitmapName` `C`
- `GetUpBitmapName` `C`
- `GetUpRoundBitmapName` `C`
- `GetUseDamageType` `C`
- `GetWarmUpEffectAttachPoint` `C`
- `GetWarmUpEffectName` `C`
- `GetWarmUpSoundName` `C`
- `GetWarmupFxPakName` `C`
- `GetWeaponDamagePct` `C`
- `GetWeaponEnchantment` `C`
- `IncludeRacialDamage` `C`
- `IsAchievementTracked` `C`
- `IsBitmapRound` `C`
- `IsExclusivePotionSkill` `C`
- `IsExclusiveSkill` `C`
- `IsQualifyingDamage` `C`
- `IsQualifyingDamage` `C`
- `LoadProfile` `V`
- `OnGenericEntityInitialUpdate`
- `ResolveEnum_TargetingMode` `S`
- `SkillProfile`
- `SkillProfile`
- ``vftable'`
- `operator=`
- `~SkillProfile` `V`

### `SkillSecondary` (Game.dll, 19)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsActive` `VC`
- `Load` `V`
- `OnHitSecondaryActivation` `V`
- `PrimaryActivateNow` `V`
- `PrimaryStop` `V`
- `RTTI_new` `S`
- `RequiresSpecialActivation` `C`
- `SkillSecondary`
- `SkillSecondary`
- `SkillSpawnObject` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary` `V`

### `SkillSecondary_AttackProjectileAreaEffect` (Game.dll, 15)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetNumProjectiles` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `ProjectileLimit` `V`
- `RTTI_new` `S`
- `SkillSecondary_AttackProjectileAreaEffect`
- `SkillSecondary_AttackProjectileAreaEffect`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_AttackProjectileAreaEffect` `V`

### `SkillSecondary_AttackProjectileOrbiting` (Game.dll, 18)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `ProjectileLimit` `V`
- `RTTI_new` `S`
- `ResolveEnum_ProjectileStart` `S`
- `SkillSecondary_AttackProjectileOrbiting`
- `SkillSecondary_AttackProjectileOrbiting`
- `TargetInformation` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_AttackProjectileOrbiting` `V`

### `SkillSecondary_AttackProjectileRing` (Game.dll, 15)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `SkillSecondary_AttackProjectileRing`
- `SkillSecondary_AttackProjectileRing`
- `SkillSpawnObject` `V`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_AttackProjectileRing` `V`

### `SkillSecondary_AttackRadius` (Game.dll, 17)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `SkillSecondary_AttackRadius`
- `SkillSecondary_AttackRadius`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_AttackRadius` `V`

### `SkillSecondary_AttackRadiusLightning` (Game.dll, 14)

- `ActivateNow` `V`
- `CreateLightning` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnLightningComplete` `V`
- `RTTI_new` `S`
- `SkillSecondary_AttackRadiusLightning`
- `SkillSecondary_AttackRadiusLightning`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_AttackRadiusLightning` `V`

### `SkillSecondary_Bonus` (Game.dll, 10)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillSecondary_Bonus`
- `SkillSecondary_Bonus`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_Bonus` `V`

### `SkillSecondary_BuffAttackRadiusDuration` (Game.dll, 24)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `IsTrackable` `VC`
- `PrimaryStop` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SkillSecondary_BuffAttackRadiusDuration`
- `SkillSecondary_BuffAttackRadiusDuration`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~SkillSecondary_BuffAttackRadiusDuration` `V`

### `SkillSecondary_BuffRadius` (Game.dll, 17)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `IsBuffActive` `VC`
- `Load` `V`
- `PrimaryActivateNow` `V`
- `PrimaryStop` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SkillSecondary_BuffRadius`
- `SkillSecondary_BuffRadius`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~SkillSecondary_BuffRadius` `V`

### `SkillSecondary_BuffSelfDuration` (Game.dll, 18)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBuffActive` `VC`
- `IsPassive` `VC`
- `Load` `V`
- `PrimaryStop` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SkillSecondary_BuffSelfDuration`
- `SkillSecondary_BuffSelfDuration`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_BuffSelfDuration` `V`

### `SkillSecondary_ChainBonus` (Game.dll, 17)

- `ActivateNow` `V`
- `CreateChainEffect`
- `DecayChain`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `GrowChain`
- `OnChainEffectComplete`
- `RTTI_new` `S`
- `SkillSecondary_ChainBonus`
- `SkillSecondary_ChainBonus`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_ChainBonus` `V`

### `SkillSecondary_ChainLightning` (Game.dll, 19)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CreateLightning`
- `DecayChain`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `GrowChain`
- `Load` `V`
- `OnLightningComplete`
- `RTTI_new` `S`
- `SkillSecondary_ChainLightning`
- `SkillSecondary_ChainLightning`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_ChainLightning` `V`

### `SkillSecondary_ForkLightning` (Game.dll, 17)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CreateLightning`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `MonitorFork`
- `OnLightningComplete`
- `RTTI_new` `S`
- `SkillSecondary_ForkLightning`
- `SkillSecondary_ForkLightning`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_ForkLightning` `V`

### `SkillSecondary_OnHitBuffRadius` (Game.dll, 17)

- `CreateUINextSpecializedText1` `VC`
- `CreateUISpecializedText1` `VC`
- `GetActivationChance` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `IsSkillOnHitActive` `VC`
- `Load` `V`
- `OnHitSecondaryActivation` `V`
- `RTTI_new` `S`
- `SkillSecondary_OnHitBuffRadius`
- `SkillSecondary_OnHitBuffRadius`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_OnHitBuffRadius` `V`

### `SkillSecondary_OnKillAttackRadius` (Game.dll, 15)

- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `OnEnemyDeath` `V`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `SkillSecondary_OnKillAttackRadius`
- `SkillSecondary_OnKillAttackRadius`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_OnKillAttackRadius` `V`

### `SkillSecondary_PetModifier` (Game.dll, 20)

- `ActivateNow` `V`
- `CollectModifierPetChanges` `VC`
- `GetCooldownData` `C`
- `GetPetSkillName` `C`
- `GetRTTIClassInfo` `VC`
- `GetSkillModifierData` `VC`
- `GetSkillProfile` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `RTTI_new` `S`
- `ResetPetSkill`
- `SkillSecondary_PetModifier`
- `SkillSecondary_PetModifier`
- `UpdatePetSkill`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_PetModifier` `V`

### `SkillSecondary_PetSpawn` (Game.dll, 13)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSecondary_PetSpawn`
- `SkillSecondary_PetSpawn`
- `SkillSpawnObject` `V`
- `SpawnPet`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_PetSpawn` `V`

### `SkillSecondary_TargetedSpawnPet` (Game.dll, 14)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSecondary_TargetedSpawnPet`
- `SkillSecondary_TargetedSpawnPet`
- `SkillSpawnObject` `V`
- `SpawnPet` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_TargetedSpawnPet` `V`

### `SkillSecondary_Tether` (Game.dll, 18)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargets` `V`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `SkillSecondary_Tether`
- `SkillSecondary_Tether`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SkillSecondary_Tether` `V`

### `SkillServicesBase` (Game.dll, 6)

- `GetBuffList` `VC`
- `SkillServicesBase`
- `SkillServicesBase`
- ``vftable'`
- `operator=`
- `~SkillServicesBase` `V`

### `SkillServices_Character` (Game.dll, 35)

- `AddAutoCastController` `V`
- `AddRefreshCooldownController` `V`
- `AddRefreshDurationController` `V`
- `AttackingEnemy` `V`
- `CalculateAllocatedMemory` `VC`
- `CastingBuff` `V`
- `CastingDebuf` `V`
- `ClearCooldownBlock` `V`
- `CreateUpdateSkillBuff` `V`
- `Dead` `V`
- `EnemyDeath` `V`
- `GetBuffList` `VC`
- `HitByEnemy` `V`
- `IsAlreadyRegistered` `V`
- `IsRefreshSkillCooldownAlreadyRegistered` `V`
- `IsRefreshSkillDurationAlreadyRegistered` `V`
- `ModifyChanceToRun` `V`
- `RemoveAutoCastController` `V`
- `RemoveRefreshCooldownController` `V`
- `RemoveRefreshDurationController` `V`
- `RemoveSkillBuff` `V`
- `SendComboChargeUpdate` `V`
- `SendCreateProjectile` `V`
- `SendSkillActiveUpdate` `V`
- `SendSkillAugmentUpdate` `V`
- `SendSkillStateUpdate` `V`
- `SendSpawnObject` `V`
- `SendTargetResult` `V`
- `SetParent` `V`
- `SkillServices_Character`
- `SkillServices_Character`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~SkillServices_Character` `V`

### `SkillState` (Game.dll, 6)

- `RestoreState`
- `SaveState` `C`
- `SkillState`
- `StreamProperties`
- `operator=`
- `~SkillState`

### `SkillStateUpdateConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SkillStateUpdateConfigCmd`
- `SkillStateUpdateConfigCmd`
- ``vftable'`
- `operator=`
- `~SkillStateUpdateConfigCmd` `V`

### `SkillStateUpdateConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SkillStateUpdateConfigCmdPacket`
- `SkillStateUpdateConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SkillStateUpdateConfigCmdPacket` `V`

### `SkillTargetResultConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SkillTargetResultConfigCmd`
- `SkillTargetResultConfigCmd`
- ``vftable'`
- `operator=`
- `~SkillTargetResultConfigCmd` `V`

### `SkillTargetResultConfigCmdPacket` (Game.dll, 14)

- `CopyInbound` `V`
- `Flag_ChainAttack` `S`
- `Flag_FumbledOrMissed` `S`
- `Flag_HitIteration` `S`
- `Flag_MultipleTargets` `S`
- `Flag_SingleTarget` `S`
- `Flag_UseWeapon` `S`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SkillTargetResultConfigCmdPacket`
- `SkillTargetResultConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SkillTargetResultConfigCmdPacket` `V`

### `Skill_AktaiosLightOfRa` (Game.dll, 10)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AktaiosLightOfRa`
- `Skill_AktaiosLightOfRa`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AktaiosLightOfRa` `V`

### `Skill_AktaiosMirage` (Game.dll, 17)

- `GetExtentsMultiplier` `VC`
- `GetNewCasterCoords` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PostPetSpawned` `V`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `SetCasterCoords` `V`
- `SkillSpawnObject` `V`
- `Skill_AktaiosMirage`
- `Skill_AktaiosMirage`
- `SwapCasterWithPet` `VC`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AktaiosMirage` `V`

### `Skill_AttackBuff` (Game.dll, 14)

- `ActivateNow` `V`
- `CanModifyChanceToRun` `VC`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackBuff`
- `Skill_AttackBuff`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackBuff` `V`

### `Skill_AttackBuffRadius` (Game.dll, 14)

- `ActivateNow` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackBuffRadius`
- `Skill_AttackBuffRadius`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackBuffRadius` `V`

### `Skill_AttackChain` (Game.dll, 16)

- `ActivateNow` `V`
- `CreateLightning`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `GrowChain`
- `OnLightningComplete`
- `RTTI_new` `S`
- `Skill_AttackChain`
- `Skill_AttackChain`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackChain` `V`

### `Skill_AttackInherent` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackInherent`
- `Skill_AttackInherent`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackInherent` `V`

### `Skill_AttackLineFan` (Game.dll, 19)

- `ActivateNow` `V`
- `FilterTargets`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTextureWidth`
- `GetValidTarget` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackLineFan`
- `Skill_AttackLineFan`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackLineFan` `V`

### `Skill_AttackPathCharge` (Game.dll, 20)

- `ActivateNow` `V`
- `CreateVisualEffect` `V`
- `GetGamepadRange` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_AttackPathCharge`
- `Skill_AttackPathCharge`
- `SwipeAction` `V`
- `TargetResult` `V`
- `Update` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackPathCharge` `V`

### `Skill_AttackPattern` (Game.dll, 19)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnDestroy` `V`
- `PickCirclePoints`
- `PickGridPoints`
- `PickLinePoints`
- `PickRingPoints`
- `RTTI_new` `S`
- `ResolveEnum_Pattern` `S`
- `SkillSpawnObject` `V`
- `Skill_AttackPattern`
- `Skill_AttackPattern`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackPattern` `V`

### `Skill_AttackProjectile` (Game.dll, 13)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetSkillGroup` `VC`
- `GetStaticClassInfo` `S`
- `ProjectileLimit` `V`
- `RTTI_new` `S`
- `Skill_AttackProjectile`
- `Skill_AttackProjectile`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectile` `V`

### `Skill_AttackProjectileAreaEffect` (Game.dll, 16)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `DoSpecialProjectileGo` `V`
- `GetLaunchPoint` `VC`
- `GetNumProjectiles` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSpecialProjectileName` `V`
- `GetStaticClassInfo` `S`
- `ProjectileLimit` `V`
- `RTTI_new` `S`
- `Skill_AttackProjectileAreaEffect`
- `Skill_AttackProjectileAreaEffect`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileAreaEffect` `V`

### `Skill_AttackProjectileBurst` (Game.dll, 16)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetSkillGroup` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackProjectileBurst`
- `Skill_AttackProjectileBurst`
- `StopSkill` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileBurst` `V`

### `Skill_AttackProjectileChain` (Game.dll, 14)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackProjectileChain`
- `Skill_AttackProjectileChain`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileChain` `V`

### `Skill_AttackProjectileDebuf` (Game.dll, 13)

- `CanModifyChanceToRun` `VC`
- `GetProjectileName` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackProjectileDebuf`
- `Skill_AttackProjectileDebuf`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileDebuf` `V`

### `Skill_AttackProjectileDrop` (Game.dll, 16)

- `ActivateNow` `V`
- `AddProjectileModifier` `VC`
- `GetLaunchPoint` `VC`
- `GetLaunchPointAbove` `VC`
- `GetNumProjectiles` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnTargetInvalid` `VC`
- `RTTI_new` `S`
- `Skill_AttackProjectileDrop`
- `Skill_AttackProjectileDrop`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileDrop` `V`

### `Skill_AttackProjectileFan` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `RTTI_new` `S`
- `Skill_AttackProjectileFan`
- `Skill_AttackProjectileFan`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileFan` `V`

### `Skill_AttackProjectileMultiHit` (Game.dll, 12)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackProjectileMultiHit`
- `Skill_AttackProjectileMultiHit`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileMultiHit` `V`

### `Skill_AttackProjectileOrbiting` (Game.dll, 14)

- `ActivateNow` `V`
- `AddProjectileModifier` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `ResolveEnum_ProjectileStart` `S`
- `Skill_AttackProjectileOrbiting`
- `Skill_AttackProjectileOrbiting`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileOrbiting` `V`

### `Skill_AttackProjectileRing` (Game.dll, 14)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_AttackProjectileRing`
- `Skill_AttackProjectileRing`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileRing` `V`

### `Skill_AttackProjectileSpawnPet` (Game.dll, 18)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PostPetSpawned` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_AttackProjectileSpawnPet`
- `Skill_AttackProjectileSpawnPet`
- `SpawnPet` `V`
- `StopSkill` `V`
- `TargetInformation` `V`
- `UnInstall` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackProjectileSpawnPet` `V`

### `Skill_AttackRadius` (Game.dll, 21)

- `ActivateNow` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SetAvailability` `V`
- `ShouldWarmUpBeforeAttack` `V`
- `Skill_AttackRadius`
- `Skill_AttackRadius`
- `TargetResult` `V`
- `Update` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadius` `V`

### `Skill_AttackRadiusDisengage` (Game.dll, 13)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `RTTI_new` `S`
- `ReverseRotateDirection` `VC`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_AttackRadiusDisengage`
- `Skill_AttackRadiusDisengage`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusDisengage` `V`

### `Skill_AttackRadiusGrow` (Game.dll, 33)

- `ActivateNow` `V`
- `ApplyCastVisualEffects` `V`
- `CalculateDPS` `V`
- `CanInterrupt` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `EndAction` `V`
- `GetManaCostPeriod` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRadiusOverride` `VC`
- `GetSkillModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `GetWarmUpWasActive` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackRadiusGrow`
- `Skill_AttackRadiusGrow`
- `StartAction` `V`
- `StopEmitting` `V`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusGrow` `V`

### `Skill_AttackRadiusLeap` (Game.dll, 22)

- `ActivateNow` `V`
- `Cancel` `V`
- `CreateVisualEffect` `V`
- `EndAction` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetValidTarget` `VC`
- `GetWarmUpWasActive` `VC`
- `OnPathFailed` `VC`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_AttackRadiusLeap`
- `Skill_AttackRadiusLeap`
- `StopSkill` `V`
- `TargetResult` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusLeap` `V`

### `Skill_AttackRadiusLightning` (Game.dll, 15)

- `ActivateNow` `V`
- `CreateLightning` `V`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `OnLightningComplete` `V`
- `RTTI_new` `S`
- `Skill_AttackRadiusLightning`
- `Skill_AttackRadiusLightning`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusLightning` `V`

### `Skill_AttackRadiusLightning2` (Game.dll, 16)

- `ActivateNow` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnLightningComplete`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `Skill_AttackRadiusLightning2`
- `Skill_AttackRadiusLightning2`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusLightning2` `V`

### `Skill_AttackRadiusLightningSpawnPet` (Game.dll, 20)

- `CreateUIAlternateNextPetText` `VC`
- `CreateUIAlternatePetText` `VC`
- `CreateUINextPetText` `VC`
- `CreateUIPetText` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnLightningComplete` `V`
- `PostPetSpawned` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_AttackRadiusLightningSpawnPet`
- `Skill_AttackRadiusLightningSpawnPet`
- `SpawnPet`
- `StopSkill` `V`
- `UnInstall` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusLightningSpawnPet` `V`

### `Skill_AttackRadiusSpin` (Game.dll, 35)

- `ActivateNow` `V`
- `ApplyCastVisualEffects` `V`
- `CalculateDPS` `V`
- `CanInterrupt` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `EndAction` `V`
- `GetAllowMove` `VC`
- `GetGamepadRange` `VC`
- `GetManaCostPeriod` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSkillModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `GetWarmUpWasActive` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackRadiusSpin`
- `Skill_AttackRadiusSpin`
- `StartAction` `V`
- `StartMove` `V`
- `StopSkill` `V`
- `StopSpinning` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusSpin` `V`

### `Skill_AttackRadiusTeleport` (Game.dll, 21)

- `ActivateNow` `V`
- `Cancel` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetValidTarget` `VC`
- `GetWarmUpWasActive` `VC`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `ShouldWarmUpBeforeAttack` `V`
- `Skill_AttackRadiusTeleport`
- `Skill_AttackRadiusTeleport`
- `StopSkill` `V`
- `TargetResult` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackRadiusTeleport` `V`

### `Skill_AttackSpell` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackSpell`
- `Skill_AttackSpell`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpell` `V`

### `Skill_AttackSpellBeam` (Game.dll, 34)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CanInterrupt` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `DestroyBeams`
- `EndAction` `V`
- `GetManaCostPeriod` `VC`
- `GetMaxBeamLength`
- `GetRTTIClassInfo` `VC`
- `GetRange` `VC`
- `GetSkillModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetTargets` `V`
- `GetValidTarget` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `OnMoved` `V`
- `PlaceEffects`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackSpellBeam`
- `Skill_AttackSpellBeam`
- `StartAction` `V`
- `StartEndEffects`
- `StopEmitting` `V`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- `UpdateBeams`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellBeam` `V`

### `Skill_AttackSpellChaos` (Game.dll, 15)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackSpellChaos`
- `Skill_AttackSpellChaos`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellChaos` `V`

### `Skill_AttackSpellChaosSpawnPet` (Game.dll, 22)

- `CreateUIAlternateNextPetText` `VC`
- `CreateUIAlternatePetText` `VC`
- `CreateUINextPetLimitText` `VC`
- `CreateUINextPetText` `VC`
- `CreateUIPetLimitText` `VC`
- `CreateUIPetText` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PostPetSpawned` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_AttackSpellChaosSpawnPet`
- `Skill_AttackSpellChaosSpawnPet`
- `SpawnPet`
- `StopSkill` `V`
- `TargetResult` `V`
- `UnInstall` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellChaosSpawnPet` `V`

### `Skill_AttackSpellCone` (Game.dll, 32)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CanInterrupt` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `EndAction` `V`
- `GetEndWidth` `C`
- `GetManaCostPeriod` `VC`
- `GetMaxConeLength`
- `GetMaxRange` `C`
- `GetRTTIClassInfo` `VC`
- `GetRange` `VC`
- `GetSkillModifierData` `VC`
- `GetStartWidth` `C`
- `GetStaticClassInfo` `S`
- `GetTargets` `V`
- `GetValidTarget` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `OnMoved` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackSpellCone`
- `Skill_AttackSpellCone`
- `StartAction` `V`
- `StopEmitting` `V`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~Skill_AttackSpellCone` `V`

### `Skill_AttackSpellDrain` (Game.dll, 30)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `CanInterrupt` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `EndAction` `V`
- `FindNewTargets`
- `GetManaCostPeriod` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRange` `VC`
- `GetSkillModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `IsRunning` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `OnMoved` `V`
- `PlayEndSound`
- `PreLoad` `V`
- `RTTI_new` `S`
- `Skill_AttackSpellDrain`
- `Skill_AttackSpellDrain`
- `StartAction` `V`
- `StopEmitting` `V`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellDrain` `V`

### `Skill_AttackSpellReflectLink` (Game.dll, 25)

- `ActivateNow` `V`
- `CalculateDPS` `V`
- `GetDamageReductionPercent` `C`
- `GetMaxDuration` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsTrackable` `VC`
- `Load` `V`
- `ModifyDamage` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReflectLinkDamage` `C`
- `Skill_AttackSpellReflectLink`
- `Skill_AttackSpellReflectLink`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellReflectLink` `V`

### `Skill_AttackSpellTeleport` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackSpellTeleport`
- `Skill_AttackSpellTeleport`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellTeleport` `V`

### `Skill_AttackSpellTeleportSelf` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackSpellTeleportSelf`
- `Skill_AttackSpellTeleportSelf`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackSpellTeleportSelf` `V`

### `Skill_AttackTelekinesis` (Game.dll, 31)

- `ActivateNow` `V`
- `Cancel` `V`
- `ClaimObject`
- `EndAction` `V`
- `FindObjectToThrow`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Install` `V`
- `IsTelkineFinished` `C`
- `LiftEntity`
- `Load` `V`
- `MoveBeam`
- `MoveEntity`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReleaseObject`
- `RetrieveObject`
- `SetAvailability` `V`
- `Skill_AttackTelekinesis`
- `Skill_AttackTelekinesis`
- `StopSkill` `V`
- `SwipeAction` `V`
- `TargetResult` `V`
- `ThrowObject`
- `UnInstall` `V`
- `Update` `V`
- `WobbleCoords` `C`
- `WobbleEntity`
- ``vftable'`
- `classInfo` `S`
- `~Skill_AttackTelekinesis` `V`

### `Skill_AttackWave` (Game.dll, 22)

- `ActivateNow` `V`
- `CreateUINextSpecializedText2` `VC`
- `CreateUISpecializedText2` `VC`
- `CreateVisualEffect` `V`
- `GetEffect` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ResolveEnum_Source` `S`
- `RunTargeting`
- `Skill_AttackWave`
- `Skill_AttackWave`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~Skill_AttackWave` `V`

### `Skill_AttackWeapon` (Game.dll, 21)

- `ActivateNow` `V`
- `AddProjectileModifier` `VC`
- `Cancel` `V`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWarmUpWasActive` `VC`
- `RTTI_new` `S`
- `Skill_AttackWeapon`
- `Skill_AttackWeapon`
- `TargetResult` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackWeapon` `V`

### `Skill_AttackWeaponBlink` (Game.dll, 12)

- `Cancel` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_AttackWeaponBlink`
- `Skill_AttackWeaponBlink`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackWeaponBlink` `V`

### `Skill_AttackWeaponCharge` (Game.dll, 18)

- `ActivateNow` `V`
- `Cancel` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `MaxSkillDistance` `V`
- `OnPathFailed` `VC`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_AttackWeaponCharge`
- `Skill_AttackWeaponCharge`
- `StopSkill` `V`
- `WarmUpEnd` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackWeaponCharge` `V`

### `Skill_AttackWeaponRangedSpread` (Game.dll, 11)

- `ExecuteRangedAttack` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_AttackWeaponRangedSpread`
- `Skill_AttackWeaponRangedSpread`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_AttackWeaponRangedSpread` `V`

### `Skill_BuffAttackRadiusDuration` (Game.dll, 14)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `RTTI_new` `S`
- `Skill_BuffAttackRadiusDuration`
- `Skill_BuffAttackRadiusDuration`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffAttackRadiusDuration` `V`

### `Skill_BuffAttackRadiusLightning` (Game.dll, 14)

- `CalculateDPS` `V`
- `CreateUINextSpecializedText2` `VC`
- `CreateUISpecializedText2` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_BuffAttackRadiusLightning`
- `Skill_BuffAttackRadiusLightning`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffAttackRadiusLightning` `V`

### `Skill_BuffAttackRadiusToggled` (Game.dll, 32)

- `ActivateNow` `V`
- `AutoActivateNow` `V`
- `CalculateDPS` `V`
- `CanModifyChanceToRun` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetTrackableType` `VC`
- `InitializeBuff` `V`
- `Install` `V`
- `IsBuffActive` `VC`
- `IsTrackable` `VC`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_BuffAttackRadiusToggled`
- `Skill_BuffAttackRadiusToggled`
- `StopSkill` `V`
- `TargetResult` `V`
- `UnInstall` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffAttackRadiusToggled` `V`

### `Skill_BuffOther` (Game.dll, 14)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTotalSkillAttribute` `VC`
- `IsValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_BuffOther`
- `Skill_BuffOther`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffOther` `V`

### `Skill_BuffRadius` (Game.dll, 27)

- `ActivateNow` `V`
- `CollectPassiveCharAttributes` `VC`
- `CreateProjectile` `V`
- `CreateVisualEffect` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetTotalSkillAttribute` `VC`
- `HasIconDisplay` `VC`
- `IsActive` `VC`
- `IsBuffActive` `VC`
- `IsTrackable` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `RemoveSelfBuff` `V`
- `Skill_BuffRadius`
- `Skill_BuffRadius`
- `StopSkill` `V`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- `UpdateSkillController` `V`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffRadius` `V`

### `Skill_BuffRadiusToggled` (Game.dll, 16)

- `ActivateNow` `V`
- `AutoActivateNow` `V`
- `CanModifyChanceToRun` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTargetRadiusTag` `VC`
- `GetTrackableType` `VC`
- `HasIconDisplay` `VC`
- `IsTrackable` `VC`
- `RTTI_new` `S`
- `Skill_BuffRadiusToggled`
- `Skill_BuffRadiusToggled`
- `UpdateSkillController` `V`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffRadiusToggled` `V`

### `Skill_BuffSelfColossus` (Game.dll, 13)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Scale` `V`
- `Skill_BuffSelfColossus`
- `Skill_BuffSelfColossus`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfColossus` `V`

### `Skill_BuffSelfDuration` (Game.dll, 17)

- `ActivateNow` `V`
- `AddTimeToLive` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `RTTI_new` `S`
- `RemoveSelfBuff` `V`
- `Skill_BuffSelfDuration`
- `Skill_BuffSelfDuration`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfDuration` `V`

### `Skill_BuffSelfImmobilize` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_BuffSelfImmobilize`
- `Skill_BuffSelfImmobilize`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Skill_BuffSelfImmobilize` `V`

### `Skill_BuffSelfInvulnerable` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `Skill_BuffSelfInvulnerable`
- `Skill_BuffSelfInvulnerable`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfInvulnerable` `V`

### `Skill_BuffSelfShield` (Game.dll, 18)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `ModifyDamage` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SetAvailability` `V`
- `Skill_BuffSelfShield`
- `Skill_BuffSelfShield`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfShield` `V`

### `Skill_BuffSelfSick` (Game.dll, 13)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_BuffSelfSick`
- `Skill_BuffSelfSick`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfSick` `V`

### `Skill_BuffSelfToggled` (Game.dll, 36)

- `ActivateNow` `V`
- `AutoActivateNow` `V`
- `BonusTime` `S`
- `CanModifyChanceToRun` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveConversionAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `InitializeBuff` `V`
- `Install` `V`
- `IsBuffActive` `VC`
- `IsTrackable` `VC`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SkillLevelChange` `V`
- `Skill_BuffSelfToggled`
- `Skill_BuffSelfToggled`
- `StopSkill` `V`
- `UnInstall` `V`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_BuffSelfToggled` `V`

### `Skill_CerberusGeysers` (Game.dll, 13)

- `ActivateMarker`
- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadResources` `V`
- `RTTI_new` `S`
- `Skill_CerberusGeysers`
- `Skill_CerberusGeysers`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_CerberusGeysers` `V`

### `Skill_ChargePotion` (Game.dll, 30)

- `ActivateNow` `V`
- `CreateUIParameterText` `VC`
- `CreateUITrackerText` `V`
- `EndCooldown` `V`
- `GetMaskBottomOffset` `C`
- `GetMaskTopOffset` `C`
- `GetPotionBitmap` `C`
- `GetPotionMask` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTotalSkillAttribute` `VC`
- `GetTrackableIcon` `VC`
- `GetTrackableType` `VC`
- `IsTrackable` `VC`
- `Load` `V`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `Skill_ChargePotion`
- `Skill_ChargePotion`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- `UpdatePotionBitmap`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_ChargePotion` `V`

### `Skill_ChargedBuffOther` (Game.dll, 15)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementCharge`
- `RTTI_new` `S`
- `ResetCharge`
- `Skill_ChargedBuffOther`
- `Skill_ChargedBuffOther`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_ChargedBuffOther` `V`

### `Skill_CharonGeysers` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadResources` `V`
- `RTTI_new` `S`
- `Skill_CharonGeysers`
- `Skill_CharonGeysers`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_CharonGeysers` `V`

### `Skill_DefensiveGround` (Game.dll, 9)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_DefensiveGround`
- `Skill_DefensiveGround`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_DefensiveGround` `V`

### `Skill_DefensiveLine` (Game.dll, 12)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `ResolveEnum_SpawnDirection` `S`
- `Skill_DefensiveLine`
- `Skill_DefensiveLine`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_DefensiveLine` `V`

### `Skill_DefensiveWall` (Game.dll, 16)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `ReleaseWalls` `V`
- `SkillSpawnObject` `V`
- `Skill_DefensiveWall`
- `Skill_DefensiveWall`
- `SpawnWall` `V`
- `StopSkill` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_DefensiveWall` `V`

### `Skill_DispelMagic` (Game.dll, 16)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_DispelMagic`
- `Skill_DispelMagic`
- `TargetFriendInformation` `V`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_DispelMagic` `V`

### `Skill_DropProjectileTelekinesis` (Game.dll, 13)

- `DoSpecialProjectileGo` `V`
- `GetLaunchPoint` `VC`
- `GetNumProjectiles` `VC`
- `GetRTTIClassInfo` `VC`
- `GetSpecialProjectileName` `V`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_DropProjectileTelekinesis`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_DropProjectileTelekinesis` `V`

### `Skill_E3FauxAttack` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_E3FauxAttack`
- `Skill_E3FauxAttack`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_E3FauxAttack` `V`

### `Skill_Evade` (Game.dll, 15)

- `ActivateNow` `V`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTotalSkillAttribute` `VC`
- `RTTI_new` `S`
- `Skill_Evade`
- `Skill_Evade`
- `StartAction` `V`
- `TargetResult` `V`
- `WarmUpStart` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Evade` `V`

### `Skill_GiveBonus` (Game.dll, 15)

- `ActivateNow` `V`
- `CreateProjectile` `V`
- `GetRTTIClassInfo` `VC`
- `GetRequiresLOS` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `RTTI_new` `S`
- `Skill_GiveBonus`
- `Skill_GiveBonus`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_GiveBonus` `V`

### `Skill_Kick` (Game.dll, 17)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `RTTI_new` `S`
- `Skill_Kick`
- `Skill_Kick`
- `Skill_Kick`
- `StartAction` `V`
- `SwipeAction` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `operator=`
- `~Skill_Kick` `V`

### `Skill_Mastery` (Game.dll, 13)

- `DecrementAugmentedSkillLevel` `V`
- `GetEnumeration` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementAugmentedSkillLevel` `V`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_Mastery`
- `Skill_Mastery`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Mastery` `V`

### `Skill_MeleeModifier` (Game.dll, 12)

- `ActivateModifierAttack` `V`
- `CollectModifierDamageAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_MeleeModifier`
- `Skill_MeleeModifier`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_MeleeModifier` `V`

### `Skill_MeleeModifierRadius` (Game.dll, 10)

- `ActivateModifierAttack` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_MeleeModifierRadius`
- `Skill_MeleeModifierRadius`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_MeleeModifierRadius` `V`

### `Skill_Modifier` (Game.dll, 50)

- `CollectModifierCharAttributes` `VC`
- `CollectModifierConversionAttributes` `VC`
- `CollectModifierDefenseAttributes` `VC`
- `CollectModifierFXChanges` `VC`
- `CollectModifierOffensiveDamageAttributes` `VC`
- `CollectModifierOffensiveModifierAttributes` `VC`
- `CollectModifierRacialBonusDamage` `VC`
- `CollectModifierRacialBonusDefense` `VC`
- `CollectModifierRetaliationAttributes` `VC`
- `CollectModifierRetaliationModifierAttributes` `VC`
- `CollectModifierSkillAttributes` `VC`
- `CreateUINextPetLimitText` `VC`
- `CreateUINextPetTimeToLiveText` `VC`
- `CreateUIPetLimitText` `VC`
- `CreateUIPetTimeToLiveText` `VC`
- `GetCooldownCharges` `VC`
- `GetMaxCooldownCharges` `VC`
- `GetModifierLightningName` `C`
- `GetModifierParticleEffect` `C`
- `GetModifierProjectileFX` `C`
- `GetModifierSpawnObject1` `VC`
- `GetModifierSpawnObject2` `VC`
- `GetModifierSpawnObject3` `VC`
- `GetModifierSpawnObject4` `VC`
- `GetModifierSpawnObjectTimeToLive` `VC`
- `GetModifierTargetFX` `C`
- `GetModifierWaveChanges` `C`
- `GetRTTIClassInfo` `VC`
- `GetShapeshiftMeshOverride` `VC`
- `GetSkillModifierData` `VC`
- `GetSkillModifierData` `VC`
- `GetSkillProjectileModifierData` `VC`
- `GetSkillProjectileModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetSubSkillName` `C`
- `IsSkillEnabled` `VC`
- `Load` `V`
- `OverwriteQualifyingWeapons` `C`
- `RTTI_new` `S`
- `RemoveSelfBuff` `V`
- `SetExclusiveItemSkillModifier` `V`
- `SkillLevelChange` `V`
- `Skill_Modifier`
- `Skill_Modifier`
- `UpdateCooldownCharges` `V`
- `UpdateMaxCooldownCharges` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Modifier` `V`

### `Skill_MonsterGenerator` (Game.dll, 14)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `PostPetSpawned` `V`
- `RTTI_new` `S`
- `SendSpawnPet`
- `ShouldSaveSpawnedPets` `VC`
- `SkillSpawnObject` `V`
- `Skill_MonsterGenerator`
- `Skill_MonsterGenerator`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_MonsterGenerator` `V`

### `Skill_Move` (Game.dll, 16)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `HitAction` `V`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `Skill_Move`
- `Skill_Move`
- `StartAction` `V`
- `SwipeAction` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Move` `V`

### `Skill_OnDeathSpawnActor` (Game.dll, 12)

- `ActivateOnDeathNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_OnDeathSpawnActor`
- `Skill_OnDeathSpawnActor`
- `SpawnActor` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_OnDeathSpawnActor` `V`

### `Skill_OnHitAttackRadius` (Game.dll, 28)

- `ActivateNow` `V`
- `AutoActivateNow` `V`
- `CanModifyChanceToRun` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `CreateVisualEffect` `V`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBuffActive` `VC`
- `IsSkillOnHitActive` `VC`
- `IsTrackable` `VC`
- `OnHitActivation` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_OnHitAttackRadius`
- `Skill_OnHitAttackRadius`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_OnHitAttackRadius` `V`

### `Skill_OnHitBuffSelf` (Game.dll, 30)

- `ActivateNow` `V`
- `AutoActivateNow` `V`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBuffActive` `VC`
- `IsSkillOnHitActive` `VC`
- `IsTrackable` `VC`
- `OnHitActivation` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_OnHitBuffSelf`
- `Skill_OnHitBuffSelf`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_OnHitBuffSelf` `V`

### `Skill_OrmenosChainLaser` (Game.dll, 15)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_OrmenosChainLaser`
- `Skill_OrmenosChainLaser`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_OrmenosChainLaser` `V`

### `Skill_Passive` (Game.dll, 22)

- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveConversionAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsPassive` `VC`
- `RTTI_new` `S`
- `SkillLevelChange` `V`
- `Skill_Passive`
- `Skill_Passive`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Passive` `V`

### `Skill_PassiveDualWieldWeapon` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `Skill_PassiveDualWieldWeapon`
- `Skill_PassiveDualWieldWeapon`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveDualWieldWeapon` `V`

### `Skill_PassiveOnCritBuffSelf` (Game.dll, 12)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsSkillOnCritActive` `VC`
- `IsSkillOnHitActive` `VC`
- `RTTI_new` `S`
- `Skill_PassiveOnCritBuffSelf`
- `Skill_PassiveOnCritBuffSelf`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveOnCritBuffSelf` `V`

### `Skill_PassiveOnHitBuffSelf` (Game.dll, 34)

- `AddTimeToLive` `V`
- `BonusTime` `S`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBuffActive` `VC`
- `IsPassive` `VC`
- `IsSkillOnHitActive` `VC`
- `IsTrackable` `VC`
- `ModifyDamage` `V`
- `OnHitActivation` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_PassiveOnHitBuffSelf`
- `Skill_PassiveOnHitBuffSelf`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveOnHitBuffSelf` `V`

### `Skill_PassiveOnHitBuffShield` (Game.dll, 19)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `IsTrackable` `VC`
- `ModifyDamage` `V`
- `OnHitActivation` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_PassiveOnHitBuffShield`
- `Skill_PassiveOnHitBuffShield`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveOnHitBuffShield` `V`

### `Skill_PassiveOnLifeBuffSelf` (Game.dll, 34)

- `AddTimeToLive` `V`
- `BonusTime` `S`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveConversionAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveDamageAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBuffActive` `VC`
- `IsPassive` `VC`
- `IsTrackable` `VC`
- `Load` `V`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `Skill_PassiveOnLifeBuffSelf`
- `Skill_PassiveOnLifeBuffSelf`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveOnLifeBuffSelf` `V`

### `Skill_PassiveShield` (Game.dll, 13)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Install` `V`
- `ModifyDamage` `V`
- `ProvidesInvulnerability` `VC`
- `RTTI_new` `S`
- `Skill_PassiveShield`
- `Skill_PassiveShield`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PassiveShield` `V`

### `Skill_PetAttack` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `Skill_PetAttack`
- `Skill_PetAttack`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PetAttack` `V`

### `Skill_PlayAttackAnimation` (Game.dll, 13)

- `GetCurrentLevel` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `HitAction` `V`
- `RTTI_new` `S`
- `Skill_PlayAttackAnimation`
- `Skill_PlayAttackAnimation`
- `StartAction` `V`
- `SwipeAction` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PlayAttackAnimation` `V`

### `Skill_PotionContainer` (Game.dll, 14)

- `GetCurrentLevel` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUnlockLevel` `VC`
- `RTTI_new` `S`
- `SetSkillLevel` `V`
- `SetUnlockLevel` `V`
- `Skill_PotionContainer`
- `Skill_PotionContainer`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PotionContainer` `V`

### `Skill_PotionModifier` (Game.dll, 9)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_PotionModifier`
- `Skill_PotionModifier`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_PotionModifier` `V`

### `Skill_ProjectileModifier` (Game.dll, 13)

- `CollectModifierDamageAttributes` `VC`
- `CreateImpactFx` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_ProjectileModifier`
- `Skill_ProjectileModifier`
- `TargetInformation` `V`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_ProjectileModifier` `V`

### `Skill_ProjectileTransmuter` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementAugmentedSkillLevel` `V`
- `IsBranchSkill` `VC`
- `RTTI_new` `S`
- `Skill_ProjectileTransmuter`
- `Skill_ProjectileTransmuter`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_ProjectileTransmuter` `V`

### `Skill_RefreshCooldown` (Game.dll, 15)

- `ActivateNow` `V`
- `CreateUINextSpecializedText1` `VC`
- `CreateUISpecializedText1` `VC`
- `EndCooldown` `V`
- `GetRTTIClassInfo` `VC`
- `GetRefreshTime` `C`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_RefreshCooldown`
- `Skill_RefreshCooldown`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_RefreshCooldown` `V`

### `Skill_RefreshCooldownModifier` (Game.dll, 15)

- `CreateUINextSpecializedText1` `VC`
- `CreateUISpecializedText1` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRefreshTime` `C`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnEnemyDeath` `V`
- `RTTI_new` `S`
- `Skill_RefreshCooldownModifier`
- `Skill_RefreshCooldownModifier`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_RefreshCooldownModifier` `V`

### `Skill_Shapeshift` (Game.dll, 30)

- `ActivateNow` `V`
- `AddTimeToLive` `V`
- `CreateUINextSpecializedText2` `VC`
- `CreateUISpecializedText2` `VC`
- `GetRTTIClassInfo` `VC`
- `GetReplacementMesh` `C`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `Install` `V`
- `Load` `V`
- `PreloadSkills`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `RemoveSelfBuff` `V`
- `SetAvailability` `V`
- `SetShapeshifted`
- `SkillLevelChange` `V`
- `Skill_Shapeshift`
- `Skill_Shapeshift`
- `StartAction` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `UnInstall` `V`
- `UnloadSkills`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Shapeshift` `V`

### `Skill_SpawnMegalesiosSpirit` (Game.dll, 13)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `RTTI_new` `S`
- `Skill_SpawnMegalesiosSpirit`
- `Skill_SpawnMegalesiosSpirit`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnMegalesiosSpirit` `V`

### `Skill_SpawnMiniPet` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `KillExtraPets` `V`
- `PostPetSpawned` `V`
- `RTTI_new` `S`
- `Skill_SpawnMiniPet`
- `Skill_SpawnMiniPet`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnMiniPet` `V`

### `Skill_SpawnPet` (Game.dll, 27)

- `ActivateNow` `V`
- `CanModifyChanceToRun` `VC`
- `GetExtentsMultiplier` `VC`
- `GetNewCasterCoords` `VC`
- `GetPetPen` `V`
- `GetPetReleaseType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `KillExtraPets` `V`
- `LoadResources` `V`
- `PostPetSpawned` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SetCasterCoords` `V`
- `ShouldJoinWithParent` `VC`
- `Skill_SpawnPet`
- `Skill_SpawnPet`
- `SpawnPet` `V`
- `SpawnQuestPets` `V`
- `StopSkill` `V`
- `SwapCasterWithPet` `VC`
- `UnInstall` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnPet` `V`

### `Skill_SpawnPetMonster` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_SpawnPetMonster`
- `Skill_SpawnPetMonster`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnPetMonster` `V`

### `Skill_SpawnPetTransmuter` (Game.dll, 16)

- `CreateUINextPetTimeToLiveText` `VC`
- `CreateUIPetTimeToLiveText` `VC`
- `GetModifierSpawnObjectTimeToLive` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementAugmentedSkillLevel` `V`
- `IsBranchSkill` `VC`
- `Load` `V`
- `PreLoadPet` `V`
- `RTTI_new` `S`
- `Skill_SpawnPetTransmuter`
- `Skill_SpawnPetTransmuter`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnPetTransmuter` `V`

### `Skill_SpawnQuestPet` (Game.dll, 13)

- `GetPetPen` `V`
- `GetPetReleaseType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_SpawnQuestPet`
- `Skill_SpawnQuestPet`
- `SpawnPet` `V`
- `SpawnQuestPets` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_SpawnQuestPet` `V`

### `Skill_TargetedSpawnPet` (Game.dll, 24)

- `ActivateNow` `V`
- `CanModifyChanceToRun` `VC`
- `GetGamepadRange` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `IsTrackable` `VC`
- `Load` `V`
- `PostPetSpawned` `V`
- `PreLoadResources` `V`
- `RTTI_new` `S`
- `SkillSpawnObject` `V`
- `Skill_TargetedSpawnPet`
- `Skill_TargetedSpawnPet`
- `SpawnPet` `V`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `UnInstall` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_TargetedSpawnPet` `V`

### `Skill_Teleport` (Game.dll, 12)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetValidTarget` `VC`
- `RTTI_new` `S`
- `Skill_Teleport`
- `Skill_Teleport`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Teleport` `V`

### `Skill_Transmuter` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementAugmentedSkillLevel` `V`
- `IsBranchSkill` `VC`
- `RTTI_new` `S`
- `Skill_Transmuter`
- `Skill_Transmuter`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_Transmuter` `V`

### `Skill_TurretFireControl` (Game.dll, 11)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `LoadResources` `V`
- `RTTI_new` `S`
- `Skill_TurretFireControl`
- `Skill_TurretFireControl`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_TurretFireControl` `V`

### `Skill_TyphonSkillTransfer` (Game.dll, 15)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ShouldMonsterSkillBeInterruptedWhilePursuing` `VC`
- `Skill_TyphonSkillTransfer`
- `Skill_TyphonSkillTransfer`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_TyphonSkillTransfer` `V`

### `Skill_WPAttack` (Game.dll, 29)

- `AddProjectileEffects` `VC`
- `AddProjectileModifier` `VC`
- `ClearHitIteration` `V`
- `CollectLocalConversionAttributes` `VC`
- `CollectLocalOffensiveDamageAttributes` `VC`
- `CollectLocalOffensiveModifierAttributes` `VC`
- `CollectLocalRetaliationDamagePercent` `VC`
- `CollectLocalWeaponDamage` `VC`
- `CreateTargetFx` `V`
- `GetChanceWeight` `VC`
- `GetHitIteration` `VC`
- `GetParentSkillId` `C`
- `GetRTTIClassInfo` `VC`
- `GetSkillModifierData` `VC`
- `GetStaticClassInfo` `S`
- `GetTotalSkillAttribute` `VC`
- `IncHitIteration` `V`
- `PlayHitSound` `VC`
- `PlaySwipeSound` `VC`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `SetMockParentSkillId`
- `SetParentSkillId`
- `Skill_WPAttack`
- `Skill_WPAttack`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WPAttack` `V`

### `Skill_WPAttack_AttackWave` (Game.dll, 18)

- `ActivateNow` `V`
- `GetEffect` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RunTargeting`
- `Skill_WPAttack_AttackWave`
- `Skill_WPAttack_AttackWave`
- `StartAction` `V`
- `StopSkill` `V`
- `TargetResult` `V`
- `Update` `V`
- ``vftable'`
- `classInfo` `S`
- `~Skill_WPAttack_AttackWave` `V`

### `Skill_WPAttack_BasicAttack` (Game.dll, 12)

- `ActivateNow` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IncrementComboCharge` `V`
- `RTTI_new` `S`
- `Skill_WPAttack_BasicAttack`
- `Skill_WPAttack_BasicAttack`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WPAttack_BasicAttack` `V`

### `Skill_WPAttack_ProjectileBurst` (Game.dll, 11)

- `ExecuteRangedAttack` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_WPAttack_ProjectileBurst`
- `Skill_WPAttack_ProjectileBurst`
- `StartAction` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WPAttack_ProjectileBurst` `V`

### `Skill_WPAttack_RadialCrit` (Game.dll, 17)

- `CollectLocalConversionAttributes` `VC`
- `CollectLocalOffensiveDamageAttributes` `VC`
- `CollectLocalOffensiveModifierAttributes` `VC`
- `CollectLocalRacialBonusDamage` `VC`
- `CollectLocalRetaliationDamagePercent` `VC`
- `CollectLocalWeaponDamage` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnHitTarget` `V`
- `RTTI_new` `S`
- `Skill_WPAttack_RadialCrit`
- `Skill_WPAttack_RadialCrit`
- `TargetResult` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WPAttack_RadialCrit` `V`

### `Skill_WPAttack_Radius` (Game.dll, 11)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `OnHitTarget` `V`
- `RTTI_new` `S`
- `Skill_WPAttack_Radius`
- `Skill_WPAttack_Radius`
- `StartAction` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WPAttack_Radius` `V`

### `Skill_WeaponPool_BasicAttack` (Game.dll, 9)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Skill_WeaponPool_BasicAttack`
- `Skill_WeaponPool_BasicAttack`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WeaponPool_BasicAttack` `V`

### `Skill_WeaponPool_ChargedFinale` (Game.dll, 27)

- `ActivateSecondarySkills` `V`
- `AlternateActivateSecondarySkills`
- `CalculateDPS` `V`
- `EndAction` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `HitAction` `V`
- `IsTrackable` `VC`
- `Load` `V`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `SetChargeEffect`
- `Skill_WeaponPool_ChargedFinale`
- `Skill_WeaponPool_ChargedFinale`
- `StartAction` `V`
- `StopSkill` `V`
- `SwipeAction` `V`
- `TargetResult` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- `WPPostAttackCallback` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WeaponPool_ChargedFinale` `V`

### `Skill_WeaponPool_ChargedLinear` (Game.dll, 23)

- `CreateUINextParameterText` `VC`
- `CreateUIParameterText` `VC`
- `DecrementCharge`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `IncrementCharge`
- `IsTrackable` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `ResetCharge`
- `SetChargeEffect`
- `Skill_WeaponPool_ChargedLinear`
- `Skill_WeaponPool_ChargedLinear`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- `WPPostAttackCallback` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WeaponPool_ChargedLinear` `V`

### `Skill_WeaponPool_ChargedScaling` (Game.dll, 37)

- `CollectLocalConversionAttributes` `VC`
- `CollectLocalOffensiveDamageAttributes` `VC`
- `CollectLocalOffensiveModifierAttributes` `VC`
- `CollectLocalRacialBonusDamage` `VC`
- `CollectLocalRetaliationDamagePercent` `VC`
- `CollectLocalWeaponDamage` `VC`
- `CollectPassiveCharAttributes` `VC`
- `CollectPassiveDefenseAttributes` `VC`
- `CollectPassiveOffensiveModifierAttributes` `VC`
- `CollectPassiveRacialBonusDamage` `VC`
- `CollectPassiveRacialBonusDefense` `VC`
- `CollectPassiveRetaliationAttributes` `VC`
- `CollectPassiveRetaliationModifierAttributes` `VC`
- `CollectPassiveSkillAttributes` `VC`
- `DecrementCharge`
- `GetMaxChargeScale` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetTrackableType` `VC`
- `IncrementCharge`
- `IsTrackable` `VC`
- `RTTI_new` `S`
- `ReceiveActiveUpdate` `V`
- `ResetCharge`
- `SetChargeEffect`
- `Skill_WeaponPool_ChargedScaling`
- `Skill_WeaponPool_ChargedScaling`
- `StopSkill` `V`
- `TrackableRemainingTime` `VC`
- `TrackableTotalTime` `VC`
- `Update` `V`
- `WPPostAttackCallback` `V`
- `WPPreAttackCallback` `V`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WeaponPool_ChargedScaling` `V`

### `Skill_WeaponPool_Default` (Game.dll, 10)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SetAvailability` `V`
- `Skill_WeaponPool_Default`
- `Skill_WeaponPool_Default`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~Skill_WeaponPool_Default` `V`

### `Skybox` (Engine.dll, 4)

- `AddToScene` `C`
- `Skybox`
- `operator=`
- `~Skybox`

### `SlotManager` (Game.dll, 17)

- `AddNewRing`
- `CalculateAllocatedMemory` `C`
- `CalculateSlotPosition`
- `CalculateSlotPositions`
- `CleanupRing`
- `DebugRender`
- `GetPrioritizedSlot`
- `ReleaseSlot`
- `RequestSlot`
- `SearchRingForFreeSlot`
- `SearchRingForOverrideSlot`
- `SetNumSlots`
- `SlotManager`
- `SlotManager`
- `kDefaultNumRings` `S`
- `kMaxNumRings` `S`
- `~SlotManager`

### `SmartObjectId` (Engine.dll, 13)

- `FinishReplication`
- `GetId` `C`
- `GetReplicationResult`
- `IsNullAllowed` `V`
- `MarkForReplication`
- `SetReplicationResult`
- `ShouldReplicate` `C`
- `SmartObjectId`
- `SmartObjectId`
- ``vftable'`
- `operator=`
- `operator=`
- `~SmartObjectId` `V`

### `SmartObjectIdList` (Engine.dll, 14)

- `AddId`
- `AddId`
- `AddIdList`
- `CreateId` `V`
- `GetIdList`
- `GetParent`
- `Size` `C`
- `SmartObjectIdList`
- `SmartObjectIdList`
- ``vftable'`
- `operator=`
- `operator=`
- `operator[]` `C`
- `~SmartObjectIdList` `V`

### `SmartObjectIdListNullable` (Engine.dll, 9)

- `CreateId` `V`
- `SmartObjectIdListNullable`
- `SmartObjectIdListNullable`
- `SmartObjectIdListNullable`
- ``vftable'`
- `operator=`
- `operator=`
- `operator=`
- `~SmartObjectIdListNullable` `V`

### `SmartObjectIdNullable` (Engine.dll, 9)

- `IsNullAllowed` `V`
- `SmartObjectIdNullable`
- `SmartObjectIdNullable`
- `SmartObjectIdNullable`
- ``vftable'`
- `operator=`
- `operator=`
- `operator=`
- `~SmartObjectIdNullable` `V`

### `Socket` (Engine.dll, 25)

- `ClearErrors`
- `Compress` `V`
- `Decompress` `V`
- `DumpInterfaceInfo` `V`
- `FlushSendQueue`
- `GetBytesRecvCompressed` `C`
- `GetBytesRecvUncompressed` `C`
- `GetBytesSentCompressed` `C`
- `GetBytesSentUncompressed` `C`
- `GetErrorCount`
- `GetErrors`
- `GetMTU` `C`
- `GetSocketType` `C`
- `IsShutdown` `C`
- `LogError`
- `Send`
- `SetNetworkLogging`
- `Socket`
- `Socket`
- `Update` `V`
- ``vftable'`
- `kCompressionBufferSize` `S`
- `kReceiveBufferSize` `S`
- `kSendQueueSize` `S`
- `~Socket` `V`

### `Sound` (Engine.dll, 17)

- `DecLoopCount`
- `GetTime` `V`
- `GetType` `C`
- `Is3d` `C`
- `IsPaused` `C`
- `IsPlaying` `V`
- `Set3dPosition` `V`
- `SetFalloffDistances` `V`
- `SetFinished` `V`
- `SetLoopCount`
- `SetPlaybackRate` `V`
- `Sound`
- `Sound`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~Sound` `V`

### `SoundAmbient` (Game.dll, 10)

- `GetQuickUpdateCheck` `C`
- `Load`
- `Load`
- `PreLoad`
- `SoundAmbient`
- `StartSound`
- `StopSound`
- `Update`
- `operator=`
- `~SoundAmbient`

### `SoundDescriptor` (Engine.dll, 9)

- `GetFileName` `C`
- `PreLoad`
- `SetDefault`
- `SoundDescriptor`
- `SoundDescriptor`
- `SoundDescriptor`
- `operator=`
- `operator=`
- `~SoundDescriptor`

### `SoundEntity` (Engine.dll, 20)

- `AddToWorld`
- `DestroySoundPak`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsSavedByEditor` `VC`
- `MakeSelfDeletingChild`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `SetSelfRemove`
- `SetSoundPak`
- `SoundEntity`
- `StartPlaying`
- `StopPlaying`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SoundEntity` `V`

### `SoundManager` (Engine.dll, 84)

- `AddBackgroundThreadSound`
- `CleanFadeList`
- `Disable2D`
- `Disable3D`
- `DisableDistanceCheck`
- `DisableSound`
- `DisplayDebuggingInfo`
- `DisplayStats` `C`
- `Dump`
- `Enable2D`
- `Enable3D`
- `EnableDebugging`
- `EnableDistanceCheck`
- `EnableSound`
- `EnableStats`
- `EvictOldResources`
- `FadeSound`
- `FastPause`
- `FastUnPause`
- `GetCameraLerp`
- `GetCaptureDeviceNames` `C`
- `GetCapturedAudioData`
- `GetEarsPosition`
- `GetEmptyCurrentPlayingIndex`
- `GetExternalStreamTime` `C`
- `GetFreeInstance`
- `GetInstanceFromIndex`
- `GetNumPlaying` `C`
- `GetPlayCounter`
- `GetPlayLength`
- `GetSoundDeviceNames` `C`
- `GetSoundResourceManager`
- `GetSoundResourceManager` `C`
- `GetState` `C`
- `GetVolumeSetMultiplier` `C`
- `Initialize`
- `InternalUnload`
- `IsDialogPlaying` `C`
- `IsPlaying`
- `IsValidPositionInFrustum` `C`
- `Load`
- `MasterFadeIn`
- `MasterFadeOut`
- `Pause`
- `Play2D`
- `Play3D`
- `PlayBackgroundThreadSound`
- `PlayExternalStream`
- `PlayExternalStream`
- `ProcessFinishedSounds`
- `RegisterMovingObject`
- `ReinitializeCapture`
- `RemoveMovingObject`
- `ReturnInstance`
- `SetCameraLerp`
- `SetCaptureEnabled`
- `SetEarsOrientation`
- `SetEarsPosition`
- `SetGlobalReverbType`
- `SetMasterVolume`
- `SetTestMode`
- `SetVolumeSetMultiplier`
- `Shutdown`
- `SoundManager`
- `StartStream`
- `StopAll`
- `StopExternalStream`
- `StopPlaying`
- `UnPause`
- `Unload`
- `UnloadPendingDescriptors`
- `UnloadUnreferencedResources`
- `Update`
- `UpdateAllPositions`
- `UpdateAllVolumes`
- `UpdateMusicVolumes`
- `UpdatePosition`
- `UpdateVolume`
- `WriteResourceLog`
- `_Pause`
- `_UnPause`
- ``vftable'`
- `kMaxConcurrentSounds` `S`
- `~SoundManager` `V`

### `SoundObject` (Game.dll, 15)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `OccludesPathing` `VC`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ShouldServerSpawn` `VC`
- `SoundObject`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~SoundObject` `V`

### `SoundPak` (Engine.dll, 30)

- `CalculateAllocatedMemory` `VC`
- `CalculateMemoryUsage` `VC`
- `CouldPossiblyHear` `C`
- `DoFalloffCheck`
- `EnablePauseHack`
- `FadeLastPlayedSound`
- `GetFalloff` `C`
- `GetForce2D` `C`
- `GetNumLoaded`
- `GetPlayLocation` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsPlaying`
- `Load` `V`
- `PickSound` `C`
- `Play2D` `V`
- `Play3D` `V`
- `PreLoad`
- `RTTI_new` `S`
- `SetVolume`
- `SetVolumeSet`
- `SoundPak`
- `SoundPak`
- `Stop`
- `StopTracking`
- `Track`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~SoundPak` `V`

### `SoundPeriodic` (Game.dll, 10)

- `GetQuickUpdateCheck` `C`
- `InitializeSoundPeriod`
- `Load`
- `PreLoad`
- `SoundPeriodic`
- `StartSound`
- `StopSound`
- `Update`
- `operator=`
- `~SoundPeriodic`

### `SoundResource` (Engine.dll, 8)

- `Destroy` `V`
- `GetDataSize` `C`
- `GetSoundImage` `C`
- `GetSystemMemoryUsage` `VC`
- `Initialize` `V`
- `InitializeDefault` `V`
- `SoundResource`
- `~SoundResource` `V`

### `SoundSample` (Engine.dll, 5)

- `SoundSample`
- `SoundSample`
- ``vftable'`
- `operator=`
- `~SoundSample` `V`

### `SoundStream` (Engine.dll, 15)

- `GetBytesPlayed` `VC`
- `GetBytesWritten` `VC`
- `GetStreamTime` `VC`
- `Pause` `V`
- `Play` `V`
- `Resume` `V`
- `SoundStream`
- `SoundStream`
- `Stop` `V`
- `StreamData` `V`
- `StreamData` `V`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~SoundStream` `V`

### `SoundSystem` (Engine.dll, 10)

- `SoundSystem`
- `SoundSystem`
- `StreamCloseCallback` `S`
- `StreamOpenCallback` `S`
- `StreamReadCallback` `S`
- `StreamSeekCallback` `S`
- `StreamTellCallback` `S`
- ``vftable'`
- `operator=`
- `~SoundSystem` `V`

### `SpawnAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `SpawnAction`
- `SpawnAction`
- `ToString` `VC`
- ``vftable'`
- `~SpawnAction` `V`

### `SpawnActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `SpawnActionPacket`
- `SpawnActionPacket`
- ``vftable'`
- `operator=`
- `~SpawnActionPacket` `V`

### `SpawnActorDeathHandler` (Game.dll, 14)

- `AnimationCallback` `V`
- `Execute` `V`
- `Finish` `V`
- `IsOverideAllowed` `VC`
- `IsSpawnHandler` `VC`
- `PreLoad` `V`
- `ShouldSaveState` `VC`
- `SpawnActor` `V`
- `SpawnActorDeathHandler`
- `SpawnActorDeathHandler`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~SpawnActorDeathHandler` `V`

### `SpawnActorPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SpawnActorPacket`
- `SpawnActorPacket`
- ``vftable'`
- `operator=`
- `~SpawnActorPacket` `V`

### `SpawnMyBonesDeathHandler` (Game.dll, 12)

- `AnimationCallback` `V`
- `Execute` `V`
- `IsOverideAllowed` `VC`
- `NeedsObjectId` `VC`
- `PreLoad` `V`
- `SetObjectId` `V`
- `ShouldSaveState` `VC`
- `SpawnMyBonesDeathHandler`
- `SpawnMyBonesDeathHandler`
- ``vftable'`
- `operator=`
- `~SpawnMyBonesDeathHandler` `V`

### `SpawnObjectConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `SpawnObjectConfigCmd`
- `SpawnObjectConfigCmd`
- ``vftable'`
- `operator=`
- `~SpawnObjectConfigCmd` `V`

### `SpawnObjectConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SpawnObjectConfigCmdPacket`
- `SpawnObjectConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~SpawnObjectConfigCmdPacket` `V`

### `SpawnPlayerPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SpawnPlayerPacket`
- `SpawnPlayerPacket`
- ``vftable'`
- `operator=`
- `~SpawnPlayerPacket` `V`

### `SpecialCharHandler` (Game.dll, 15)

- `CalculateAllocatedMemory` `C`
- `CreateHandler` `S`
- `Disable` `V`
- `Enable` `V`
- `GetEnabled`
- `LoadFromTable` `V`
- `PhysicsUpdate` `V`
- `PreLoad` `V`
- `Reset` `V`
- `SpecialCharHandler`
- `SpecialCharHandler`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~SpecialCharHandler` `V`

### `SpecialCharHandler_FadeAwayFromPlayer` (Game.dll, 11)

- `Disable` `V`
- `Enable` `V`
- `GetDistanceToClosestPlayer`
- `GetFadeColor` `V`
- `LoadFromTable` `V`
- `SpecialCharHandler_FadeAwayFromPlayer`
- `SpecialCharHandler_FadeAwayFromPlayer`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~SpecialCharHandler_FadeAwayFromPlayer` `V`

### `SpecialCharHandler_FadeNearPlayer` (Game.dll, 6)

- `GetFadeColor` `V`
- `SpecialCharHandler_FadeNearPlayer`
- `SpecialCharHandler_FadeNearPlayer`
- ``vftable'`
- `operator=`
- `~SpecialCharHandler_FadeNearPlayer` `V`

### `SpecialCharHandler_IcyCharacter` (Game.dll, 19)

- `AllowBone` `C`
- `Disable` `V`
- `Enable` `V`
- `End`
- `GetOverriddenTexture` `C`
- `LoadFromTable` `V`
- `OnGMIEffectDestroy` `V`
- `OnGMIEffectFinished` `V`
- `PhysicsUpdate` `V`
- `PreLoad` `V`
- `Reset` `V`
- `SpecialCharHandler_IcyCharacter`
- `SpecialCharHandler_IcyCharacter`
- `Start`
- `StoreOverriddenTexture`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `~SpecialCharHandler_IcyCharacter` `V`

### `SpecialFX` (Game.dll, 14)

- `DeleteFx` `V`
- `GetScale` `C`
- `LoadFX` `V`
- `PreLoad`
- `ReleaseFx` `V`
- `Reset`
- `SetScale`
- `SpecialFX`
- `SpecialFX`
- `StartFx` `V`
- `StopFx` `V`
- ``vftable'`
- `operator=`
- `~SpecialFX` `V`

### `SpinButton` (Widget.dll, 1)

- `SetRange`

### `Splitter` (Widget.dll, 17)

- `Create`
- `GetHandle`
- `GetParent`
- `OnDestroy` `V`
- `OnMouseButton` `V`
- `OnMouseMove` `V`
- `OnNotify` `V`
- `OnPaint` `V`
- `OnSetCursor` `V`
- `OnSizeChange` `V`
- `SetChild`
- `SetLocked`
- `SetPosition`
- `SetSize`
- `SetSplitterPos`
- `Splitter`
- `UpdateLayout`

### `StartServerRespawnPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StartServerRespawnPacket`
- `StartServerRespawnPacket`
- ``vftable'`
- `operator=`
- `~StartServerRespawnPacket` `V`

### `StartShrineProxyPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StartShrineProxyPacket`
- `StartShrineProxyPacket`
- ``vftable'`
- `operator=`
- `~StartShrineProxyPacket` `V`

### `StartStopDamageEffectConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `StartStopDamageEffectConfigCmd`
- `StartStopDamageEffectConfigCmd`
- ``vftable'`
- `operator=`
- `~StartStopDamageEffectConfigCmd` `V`

### `StartStopDamageEffectConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StartStopDamageEffectConfigCmdPacket`
- `StartStopDamageEffectConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~StartStopDamageEffectConfigCmdPacket` `V`

### `StartTeleportPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StartTeleportPacket`
- `StartTeleportPacket`
- ``vftable'`
- `operator=`
- `~StartTeleportPacket` `V`

### `StaticMarker` (Game.dll, 11)

- `BindToCharacter` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBindingEnabled` `VC`
- `RTTI_new` `S`
- `StaticMarker`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~StaticMarker` `V`

### `StaticRespawner` (Game.dll, 12)

- `AppendDetailMapData` `V`
- `BindToCharacter` `V`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `IsBindingEnabled` `VC`
- `RTTI_new` `S`
- `StaticRespawner`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~StaticRespawner` `V`

### `StaticShrine` (Game.dll, 72)

- `AddSocialTarget`
- `AllKilled` `C`
- `AnimationCallback` `V`
- `AppendDetailMapData` `V`
- `ApplyReplicationData` `V`
- `BindToLocalPlayer` `V`
- `CleanseShrine` `V`
- `DeleteSocialTarget`
- `GetDevotionPoints` `C`
- `GetGameDescription` `VC`
- `GetItems` `C`
- `GetLootDropCoords` `VC`
- `GetLootDropGroup` `VC`
- `GetLootDropRadius` `VC`
- `GetMoveToPoint` `VC`
- `GetOffering1DisplayName` `C`
- `GetOffering1Id` `C`
- `GetOffering2DisplayName` `C`
- `GetOffering2Id` `C`
- `GetOffering3DisplayName` `C`
- `GetOffering3Id` `C`
- `GetRTTIClassInfo` `VC`
- `GetSocialTarget`
- `GetStaticClassInfo` `S`
- `GetXpReward` `C`
- `GoDormant`
- `GoDormantToRestored`
- `GoRestored`
- `HasProxy`
- `InitialUpdate` `V`
- `IsActiveForMainPlayer`
- `IsChatting`
- `IsChattingWithPlayer`
- `IsCleansed` `C`
- `IsLocked` `C`
- `IsOfInterest` `VC`
- `Load` `V`
- `LoadOffering1`
- `LoadOffering2`
- `LoadOffering3`
- `OccludesPathing` `VC`
- `OnConversationEnd`
- `OnDestroy` `V`
- `PlaceEffectsInWorld`
- `PlayAnimationAndFX`
- `PreLoad` `V`
- `RTTI_new` `S`
- `ReadReplicationData` `V`
- `RequestToUse` `V`
- `RestoreState` `V`
- `RunProxy`
- `SaveState` `VC`
- `SetLimitsLocation`
- `SetLocked`
- `SetShrineState`
- `SetState`
- `ShouldSaveState` `VC`
- `ShouldServerSpawn` `VC`
- `StartActiveEffect`
- `StartDormantEffect`
- `StartDormantToRestoredEffect`
- `StartRestoredEffect`
- `StaticShrine`
- `UpdateSelf` `V`
- `UpdateSocialTargetList`
- `WriteReplicationData` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~StaticShrine` `V`

### `StaticShrineStateChangeConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `StaticShrineStateChangeConfigCmd`
- `StaticShrineStateChangeConfigCmd`
- ``vftable'`
- `operator=`
- `~StaticShrineStateChangeConfigCmd` `V`

### `StaticShrineStateChangeConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StaticShrineStateChangeConfigCmdPacket`
- `StaticShrineStateChangeConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~StaticShrineStateChangeConfigCmdPacket` `V`

### `StaticTeleporter` (Game.dll, 30)

- `AppendDetailMapData` `V`
- `BindToCharacter` `V`
- `GetIntersection` `VC`
- `GetMoveToPoint` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `IsActiveForMainPlayer`
- `IsBindingEnabled` `VC`
- `IsOfInterest` `VC`
- `IsStatic` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RequestToUse` `V`
- `RestoreState` `V`
- `SaveState` `VC`
- `SetActive`
- `SetLocked`
- `ShouldSaveState` `VC`
- `StaticTeleporter`
- `StopEffects`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~StaticTeleporter` `V`

### `StatusBar` (Widget.dll, 3)

- `Create`
- `ResizeToFitParent`
- `SetText`

### `SteamAuthRequestPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SteamAuthRequestPacket`
- `SteamAuthRequestPacket`
- ``vftable'`
- `operator=`
- `~SteamAuthRequestPacket` `V`

### `SteamAuthResponsePacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SteamAuthResponsePacket`
- `SteamAuthResponsePacket`
- ``vftable'`
- `operator=`
- `~SteamAuthResponsePacket` `V`

### `SteamControllerDevice` (Engine.dll, 11)

- `AddActionSet`
- `AddBinding`
- `GetEvents` `C`
- `GetGlyphs` `C`
- `SetActionSet`
- `SteamControllerDevice`
- `SteamControllerDevice`
- `Update`
- `kReconnectPeriod` `S`
- `operator=`
- `~SteamControllerDevice`

### `SteamFriendList` (Engine.dll, 6)

- `GetFriendById` `V`
- `GetSteamId` `C`
- `OpenSteamFriendOverlay`
- `Refresh` `V`
- `SteamFriendList`
- `~SteamFriendList` `V`

### `Steamworks` (Engine.dll, 45)

- `AchievementIsUnlocked` `C`
- `AchievementReset`
- `AchievementUnlock`
- `AchievementsAreAvailable` `C`
- `CloudClear` `C`
- `CloudCopy` `C`
- `CloudDelete` `C`
- `CloudFileExists` `C`
- `CloudGetFileByIndex` `C`
- `CloudGetFileNames` `C`
- `CloudGetFileSize` `C`
- `CloudGetFileTime` `C`
- `CloudGetNumFiles` `C`
- `CloudGetStorageSpace` `C`
- `CloudRead` `C`
- `CloudWrite` `C`
- `ControllerConfigure` `C`
- `ControllerGetActionSet` `C`
- `ControllerGetAnalogData` `C`
- `ControllerGetAnalogGlyphs` `C`
- `ControllerGetAnalogHandle` `C`
- `ControllerGetDevice` `C`
- `ControllerGetDigitalData` `C`
- `ControllerGetDigitalGlyphs` `C`
- `ControllerGetDigitalHandle` `C`
- `ControllerGetTextInput` `C`
- `ControllerIsConnected` `C`
- `ControllerSetActionSet` `C`
- `ControllerShowTextInput` `C`
- `Destroy` `S`
- `Get` `S`
- `GetAnonymousId` `C`
- `GetAppId` `C`
- `GetSteamIdAsString` `C`
- `GetSteamIdAsUInt64` `C`
- `InBigPictureMode` `C`
- `Initialize`
- `IsInOfflineMode`
- `IsInitialized` `C`
- `OwnsDLC` `C`
- `Shutdown`
- `StatsCommit`
- `StatsGet` `C`
- `StatsSet` `C`
- `Update`

### `StopVideoRequestPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `StopVideoRequestPacket`
- `StopVideoRequestPacket`
- ``vftable'`
- `operator=`
- `~StopVideoRequestPacket` `V`

### `StubConnectionManager` (Engine.dll, 14)

- `HandlePacket` `V`
- `Initialize` `V`
- `SendPacket` `V`
- `SendPacketExcluding` `V`
- `SendPacketToGroup` `V`
- `SendPacketToHost` `V`
- `SendPacketToServer` `V`
- `Shutdown` `V`
- `StubConnectionManager`
- `StubConnectionManager`
- `Update` `V`
- ``vftable'`
- `operator=`
- `~StubConnectionManager` `V`

### `StubNetworkShim` (Engine.dll, 7)

- `SendCharacterAction` `V`
- `SendConfigCommand` `V`
- `StubNetworkShim`
- `StubNetworkShim`
- ``vftable'`
- `operator=`
- `~StubNetworkShim` `V`

### `StyleManager` (Engine.dll, 13)

- `GetLocalizationFontFile`
- `GetStyle`
- `GetUnscaledStyle`
- `LoadFontDirect`
- `LoadStyle`
- `ReloadAll`
- `ReloadStyle`
- `StyleManager`
- `UnloadAll`
- `operator=`
- `styleMap` `S`
- `unscaledStyleMap` `S`
- `~StyleManager`

### `SuperBossNotificationPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SuperBossNotificationPacket`
- `SuperBossNotificationPacket`
- ``vftable'`
- `operator=`
- `~SuperBossNotificationPacket` `V`

### `SurvivalModeDataPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SurvivalModeDataPacket`
- `SurvivalModeDataPacket`
- ``vftable'`
- `operator=`
- `~SurvivalModeDataPacket` `V`

### `SyncObjectivesPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `SyncObjectivesPacket`
- `SyncObjectivesPacket`
- ``vftable'`
- `operator=`
- `~SyncObjectivesPacket` `V`

### `SystemTimer` (Engine.dll, 7)

- `GetInternalTime` `VC`
- `SystemTimer`
- `SystemTimer`
- `SystemTimer`
- ``vftable'`
- `operator=`
- `operator=`

### `TabControl` (Widget.dll, 7)

- `AddPage`
- `Create`
- `GetVisiblePage` `C`
- `OnNotify` `V`
- `OnSizeChange` `V`
- `ShowSelectedTabPage`
- `ShowTab`

### `TableDepot` (Engine.dll, 6)

- `ClearCache`
- `DumpStats` `C`
- `GetLoadTable` `C`
- `LoadFile`
- `TableDepot`
- `~TableDepot` `V`

### `TakeBonusConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `TakeBonusConfigCmd`
- `TakeBonusConfigCmd`
- ``vftable'`
- `operator=`
- `~TakeBonusConfigCmd` `V`

### `TakeBonusConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TakeBonusConfigCmdPacket`
- `TakeBonusConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~TakeBonusConfigCmdPacket` `V`

### `TakeHitAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeHitAction`
- `TakeHitAction`
- `ToString` `VC`
- ``vftable'`
- `~TakeHitAction` `V`

### `TakeHitPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `TakeHitPacket`
- `TakeHitPacket`
- ``vftable'`
- `operator=`
- `~TakeHitPacket` `V`

### `TakeKnockdownAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeKnockdownAction`
- `TakeKnockdownAction`
- `ToString` `VC`
- ``vftable'`
- `~TakeKnockdownAction` `V`

### `TakeKnockdownPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `TakeKnockdownPacket`
- `TakeKnockdownPacket`
- ``vftable'`
- `operator=`
- `~TakeKnockdownPacket` `V`

### `TakeSleepAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeSleepAction`
- `TakeSleepAction`
- `ToString` `VC`
- ``vftable'`
- `~TakeSleepAction` `V`

### `TakeSleepPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `TakeSleepPacket`
- `TakeSleepPacket`
- ``vftable'`
- `operator=`
- `~TakeSleepPacket` `V`

### `TakeStunAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `TakeStunAction`
- `TakeStunAction`
- `ToString` `VC`
- ``vftable'`
- `~TakeStunAction` `V`

### `TakeStunPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `TakeStunPacket`
- `TakeStunPacket`
- ``vftable'`
- `operator=`
- `~TakeStunPacket` `V`

### `TakeTrapAction` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `TakeTrapAction`
- `TakeTrapAction`
- `ToString` `VC`
- ``vftable'`
- `~TakeTrapAction` `V`

### `TakeTrapPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `TakeTrapPacket`
- `TakeTrapPacket`
- ``vftable'`
- `operator=`
- `~TakeTrapPacket` `V`

### `TelkineDeathHandler` (Game.dll, 5)

- `TelkineDeathHandler`
- `TelkineDeathHandler`
- ``vftable'`
- `operator=`
- `~TelkineDeathHandler` `V`

### `Terrain` (Engine.dll, 84)

- `AddDirtyRect`
- `AddLayer`
- `AddOpacityDirtyRect`
- `AddToScene` `V`
- `CreateClutterIndexBuffer` `V`
- `CreateGrassDisturbance` `V`
- `CreateGrassDisturbance` `V`
- `CreateGrassGeometry` `V`
- `CreateGrassWind` `V`
- `CreateMapGeometry` `V`
- `DeleteLayer`
- `DestroyGrassGeometry` `V`
- `DestroyMapGeometry` `V`
- `GeometryBusStop` `V`
- `GeometryBusStop` `V`
- `GetBaseHeight` `C`
- `GetBlocksInDirtySet` `C`
- `GetBoxForRect` `C`
- `GetFastNormal` `VC`
- `GetHeight` `VC`
- `GetHeightInternal` `C`
- `GetHeightInterpolated` `VC`
- `GetIndexBufferForLayer` `C`
- `GetIntersection` `VC`
- `GetLayerOpacity` `VC`
- `GetLayerOpacityInterpolated` `VC`
- `GetLayerOpacityTexture` `VC`
- `GetLayerShader` `VC`
- `GetLayerTerrainType` `V`
- `GetLayerTerrainType` `VC`
- `GetLayersInBox` `VC`
- `GetNormalInterpolated` `VC`
- `GetNumTextureLayers` `VC`
- `GetObjectsInDirtySet` `C`
- `GetOffset`
- `GetPatchShader` `VC`
- `GetQuality` `VC`
- `GetRectForBox` `C`
- `GetTerrainLayer`
- `GetTerrainVertex` `C`
- `GetVertexDeclaration` `VC`
- `InternalAddLayer`
- `IsImpassable` `C`
- `IsInvisible` `VC`
- `IsPointVisible` `VC`
- `Load` `V`
- `LoadRenderData` `V`
- `MoveDownLayer`
- `MoveUpLayer`
- `New`
- `PathingFirstPass`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `PreLoad` `V`
- `PreLoadFrustums` `V`
- `ProcessDirtyOpacityRects`
- `ProcessDirtyRects`
- `ReloadGenerationDependentData` `V`
- `RenderBorder`
- `RenderImpassableAreas` `C`
- `Save` `C`
- `SaveRunTimeFormat` `C`
- `SetBaseHeight`
- `SetImpassableData`
- `SetInvisible`
- `SetLayerOpacity`
- `SetLayerTerrainType`
- `SetNumXSamples`
- `SetNumYSamples`
- `SetPassable`
- `ShowTextureComplexity`
- `SlopeImpassable`
- `Terrain`
- `Terrain`
- `Unload` `V`
- `Update` `V`
- `UpdateHeightMap`
- `UpdateLayerOpacity`
- `UpdateVertexBuffer`
- `UpdateVisibilityMap`
- ``vftable'`
- `maxGrassBufferCacheSize` `S`
- `useGrassBufferCache` `S`
- `~Terrain` `V`

### `TerrainBase` (Engine.dll, 37)

- `AddLoadedObject`
- `AddObjectToSpace`
- `ConstructBlocks`
- `CreatePhysics` `V`
- `DestroyPhysics` `V`
- `DestroyTerrainBlocks`
- `GeometryBusStop` `V`
- `GeometryBusStop` `V`
- `GetAllObjects` `C`
- `GetBoundingBox` `C`
- `GetFaceIntersection` `C`
- `GetIntersection` `C`
- `GetNumXSamples` `C`
- `GetNumYSamples` `C`
- `GetObjectsInBox` `C`
- `GetObjectsInFrustum` `C`
- `GetRegion` `VC`
- `GetTerainType`
- `InitializeBlocks`
- `IsPointInFace` `C`
- `IsPointVisible` `VC`
- `NumXBlocks` `C`
- `NumYBlocks` `C`
- `PhysicsSetup` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `RemoveObjectFromSpace`
- `TerrainBase`
- `TerrainBase`
- `Unload` `V`
- `UnloadOldObjects`
- `UpdateObjectInSpace`
- `UpdateObjects`
- ``vftable'`
- `blockSize` `S`
- `maxNumGrassPlanes` `S`
- `~TerrainBase` `V`

### `TerrainDecoration` (Game.dll, 28)

- `AddToScene` `V`
- `CreateGeometry`
- `GetOpacity`
- `GetPatchDepth` `C`
- `GetPatchIntersection`
- `GetPatchWidth` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `InitialUpdate` `V`
- `Load` `V`
- `OnAddToLevel` `V`
- `OnMoveInLevel` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `RTTI_new` `S`
- `Read` `V`
- `SetOpacity`
- `SetScale` `V`
- `SetScale` `V`
- `TerrainDecoration`
- `UpdateSelf` `V`
- `Write` `VC`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~TerrainDecoration` `V`

### `TerrainPatch` (Engine.dll, 49)

- `AddToScene`
- `CalculateEditorIntersection` `C`
- `CompareLayers`
- `CreateGeometry`
- `DestroyBuffers`
- `DestroyLayer`
- `DestroyLayers`
- `FaceAreaXZ` `C`
- `FixLayers`
- `GetDepth` `C`
- `GetGrassOffset` `C`
- `GetHeight` `C`
- `GetLayerOpacityInterpolated` `C`
- `GetNumRenderPasses` `VC`
- `GetOpacity`
- `GetPatchIntersection`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetTexture` `VC`
- `GetWidth` `C`
- `HasData` `C`
- `InitialUpdate` `V`
- `IsPointInFace` `C`
- `LogInfo` `VC`
- `MapVertex`
- `PassIndexToPass` `C`
- `PostDeviceReset`
- `PreDeviceReset`
- `Read`
- `RebuildLayers`
- `RecreateGrassBlocks`
- `RenderBase` `C`
- `RenderGrass` `C`
- `RenderGround` `C`
- `RenderPass` `VC`
- `SetCoords`
- `SetOpacity`
- `TerrainPatch`
- `TerrainPatch`
- `Update` `V`
- `UpdateEditorData`
- `Write` `C`
- ``vftable'`
- ``vftable'`
- `maxNumGrassPlanes` `S`
- `operator=`
- `~TerrainPatch` `V`

### `TerrainPatchRT` (Engine.dll, 10)

- `InitialUpdate` `V`
- `TerrainPatchRT`
- `TerrainPatchRT`
- `TerrainPatchRT`
- `Update` `V`
- ``vftable'`
- ``vftable'`
- `operator=`
- `operator=`
- `~TerrainPatchRT` `V`

### `TerrainRenderInterface` (Engine.dll, 13)

- `DrawLayerCountLabels` `VC`
- `GetBlocksInLayer` `VC`
- `GetTerrain` `VC`
- `RenderBase` `VC`
- `RenderGrass` `VC`
- `RenderGround` `VC`
- `TerrainRenderInterface`
- `TerrainRenderInterface`
- `TerrainRenderInterface`
- ``vftable'`
- `operator=`
- `operator=`
- `~TerrainRenderInterface` `V`

### `TerrainRenderInterfaceBase` (Engine.dll, 12)

- `AddToScene`
- `DrawLayerCountLabels` `VC`
- `GetClutterShader` `C`
- `GetFreeRenderBatch`
- `GetGrassShader` `C`
- `GetLayerShader` `C`
- `GetPatchShader` `C`
- `TerrainRenderInterfaceBase`
- `TerrainRenderInterfaceBase`
- ``vftable'`
- `operator=`
- `~TerrainRenderInterfaceBase` `V`

### `TerrainRenderInterfaceRT` (Engine.dll, 12)

- `GetBlocksInLayer` `VC`
- `GetTerrain` `VC`
- `RenderBase` `VC`
- `RenderGrass` `VC`
- `RenderGround` `VC`
- `TerrainRenderInterfaceRT`
- `TerrainRenderInterfaceRT`
- `TerrainRenderInterfaceRT`
- ``vftable'`
- `operator=`
- `operator=`
- `~TerrainRenderInterfaceRT` `V`

### `TerrainType` (Engine.dll, 40)

- `ClumpFind`
- `ClumpNeighbours`
- `GetActorName` `C`
- `GetAffectedByWind` `C`
- `GetBladeHeight` `C`
- `GetBladeSpacing` `C`
- `GetBladeTexture` `C`
- `GetBladeWidth` `C`
- `GetBumpMapTexture` `C`
- `GetClutterMesh` `C`
- `GetGlowMapTexture` `C`
- `GetHeightMapTexture` `C`
- `GetLayerShader` `C`
- `GetMiniMapColor` `C`
- `GetMinimumBladeOpacity` `C`
- `GetNumBladeVariations` `C`
- `GetNumBumpMaps` `C`
- `GetNumTextures` `C`
- `GetPatchShader` `C`
- `GetRTTIClassInfo` `VC`
- `GetRandomTexture`
- `GetStaticClassInfo` `S`
- `GetSurface` `C`
- `GetTexture` `C`
- `GetTransparentClutter` `C`
- `Load` `V`
- `LoadLayerShader` `C`
- `LoadTextures`
- `PerformClumping`
- `PreLoad`
- `RTTI_new` `S`
- `ReloadBumpmaps`
- `SetGrassShaderParams` `C`
- `SetShaderParams` `C`
- `TerrainType`
- `TerrainType`
- ``vftable'`
- `classInfo` `S`
- `operator=`
- `~TerrainType` `V`

### `TextBox` (Widget.dll, 2)

- `GetValue` `C`
- `SetValue`

### `TheoraVideoPlayer` (Engine.dll, 10)

- `IsPlaying` `VC`
- `Pause` `V`
- `Play` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `Render` `V`
- `Stop` `V`
- `TheoraVideoPlayer`
- `Update` `V`
- `~TheoraVideoPlayer` `V`

### `ThreadMonitor` (Engine.dll, 3)

- `Display`
- `GetThreadInfos` `C`
- `Update`

### `TickManager` (Game.dll, 7)

- `Reset`
- `Tick`
- `TickManager`
- `Update`
- `operator=`
- `operator=`
- `tickPeriod` `S`

### `Timer` (Engine.dll, 11)

- `GetElapsedTime` `C`
- `GetTime` `S`
- `GetTotalElapsedTime` `C`
- `Reset`
- `Timer`
- `Timer`
- `Timer`
- `Update`
- ``vftable'`
- `operator=`
- `operator=`

### `Toolbar` (Widget.dll, 7)

- `AddButton`
- `AddControl`
- `AddSeparator`
- `Create`
- `ResizeToFitParent`
- `SetButtonState`
- `~Toolbar` `V`

### `Tracker` (Engine.dll, 30)

- `Clear`
- `DecrementInt`
- `DecrementInt`
- `DisplayFrameStats`
- `DisplayStats`
- `GetInt` `C`
- `GetInt` `C`
- `GetProperty`
- `GetProperty` `C`
- `GetString` `C`
- `GetString` `C`
- `Getfloat` `C`
- `Getfloat` `C`
- `IncrementInt`
- `IncrementInt`
- `PushInt`
- `PushItemInfo`
- `PushString`
- `PushStringCounter`
- `Pushfloat`
- `Save`
- `SetInt`
- `SetInt`
- `SetProperty`
- `SetString`
- `Setfloat`
- `Tracker`
- `Update`
- ``vftable'`
- `~Tracker` `V`

### `TradeAddItemPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeAddItemPacket`
- `TradeAddItemPacket`
- ``vftable'`
- `operator=`
- `~TradeAddItemPacket` `V`

### `TradeCancelPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeCancelPacket`
- `TradeCancelPacket`
- ``vftable'`
- `operator=`
- `~TradeCancelPacket` `V`

### `TradeFinalizePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeFinalizePacket`
- `TradeFinalizePacket`
- ``vftable'`
- `operator=`
- `~TradeFinalizePacket` `V`

### `TradeInitiatePacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeInitiatePacket`
- `TradeInitiatePacket`
- ``vftable'`
- `operator=`
- `~TradeInitiatePacket` `V`

### `TradeManager` (Game.dll, 31)

- `AbleToTrade` `C`
- `CleanUp`
- `CloseTradeWindow`
- `CreateResources`
- `DestroyResources`
- `FinalizeTradeLocally`
- `GetHisTradeState`
- `GetIsTradeActive` `C`
- `GetMyTradeState`
- `GetTradePartner` `C`
- `HandleAddItemInbound`
- `HandleAddItemOutbound`
- `HandleCancelTradeInbound`
- `HandleCancelTradeOutbound`
- `HandleFinalizeTradeInbound`
- `HandleFinalizeTradeOutbound`
- `HandleInitiateTradeInbound`
- `HandleInitiateTradeOutbound`
- `HandleRemoveItemInbound`
- `HandleRemoveItemOutbound`
- `HandleSetGoldAmountInbound`
- `HandleSetGoldAmountOutbound`
- `OpenTradeWindow`
- `ReturnItems`
- `ShutDown`
- `TradeManager`
- `TradeManager`
- `Update`
- ``vftable'`
- `operator=`
- `~TradeManager` `V`

### `TradeRemoveItemPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeRemoveItemPacket`
- `TradeRemoveItemPacket`
- ``vftable'`
- `operator=`
- `~TradeRemoveItemPacket` `V`

### `TradeSetGoldPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TradeSetGoldPacket`
- `TradeSetGoldPacket`
- ``vftable'`
- `operator=`
- `~TradeSetGoldPacket` `V`

### `TradeState` (Game.dll, 16)

- `Clear`
- `DeleteAndCreateItemsForPlayer`
- `DestroyObjects`
- `GetFinalized` `C`
- `GetGoldAmount` `C`
- `GetInventorySack`
- `RemoveItem`
- `RemoveItems`
- `ReturnItems`
- `SetFinalized`
- `SetGoldAmount`
- `TradeState`
- `TradeState`
- ``vftable'`
- `operator=`
- `~TradeState` `V`

### `TrailEffect` (Engine.dll, 43)

- `AddToScene` `V`
- `CleanSegmentList`
- `DecrementAlpha`
- `FillPoints`
- `GetLowerPoints`
- `GetNumRenderPasses` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRenderPassBoundingBox` `VC`
- `GetRenderPassSortOrder` `VC`
- `GetShader2` `VC`
- `GetShaderStyle` `VC`
- `GetStaticClassInfo` `S`
- `GetTexture` `VC`
- `GetUpperPoints`
- `InitialUpdate` `V`
- `InternalUpdate`
- `IsActive` `C`
- `Liberate`
- `Load` `V`
- `LogInfo` `VC`
- `PreLoad` `V`
- `PulseAlpha`
- `RTTI_new` `S`
- `RenderPass` `VC`
- `ScaleSegments`
- `SetAnchorPoints`
- `SetBBox`
- `SetDefaultAnchors`
- `SetLowEfficiencyMode`
- `ShiftForRegionChange`
- `Start`
- `StartFade`
- `StartPulse`
- `Stop`
- `TrailEffect`
- `UpdateBoundingBox` `V`
- `UpdateSelf` `V`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~TrailEffect` `V`

### `TransformGizmo` (Engine.dll, 18)

- `GetCoords` `C`
- `GetDistanceToCircle` `C`
- `GetMode` `C`
- `HandleMouseButton`
- `HandleMouseMove`
- `Render` `C`
- `RenderArrow` `C`
- `RenderCircle` `C`
- `RenderCircle` `C`
- `SetCoords`
- `SetGridSpacing`
- `SetMode`
- `SetSize`
- `SnapToGrid` `C`
- `TransformGizmo`
- `UpdateAxisSelection`
- `operator=`
- `operator=`

### `TransmuteItemsConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `TransmuteItemsConfigCmd`
- `TransmuteItemsConfigCmd`
- ``vftable'`
- `operator=`
- `~TransmuteItemsConfigCmd` `V`

### `TransmuteItemsConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `TransmuteItemsConfigCmdPacket`
- `TransmuteItemsConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~TransmuteItemsConfigCmdPacket` `V`

### `TreeView` (Widget.dll, 18)

- `Clear`
- `Create`
- `EnsureVisible`
- `ExpandItem`
- `GetFirstSelectedItem` `C`
- `GetItemParam`
- `GetItemParent` `C`
- `GetItemText` `C`
- `GetNextSelectedItem` `C`
- `GetSelectedItem` `C`
- `HitTest` `C`
- `InsertItem`
- `RemoveItem`
- `SelectItem`
- `SetImageList`
- `SetMultipleSelection`
- `SetShowSelectAlways`
- `TreeView`

### `TriggerToken` (Engine.dll, 7)

- `ReadProperties`
- `StreamProperties`
- `TriggerToken`
- `TriggerToken`
- `WriteProperties` `C`
- `operator==`
- `~TriggerToken` `V`

### `Turret` (Game.dll, 9)

- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `Turret`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Turret` `V`

### `UnJoinLeaderPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UnJoinLeaderPacket`
- `UnJoinLeaderPacket`
- ``vftable'`
- `operator=`
- `~UnJoinLeaderPacket` `V`

### `UniqueIdMap` (Engine.dll, 5)

- `AddEntity`
- `Destroy` `S`
- `Get` `S`
- `GetEntity` `C`
- `RemoveEntity`

### `UpdateAltarReagentsPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UpdateAltarReagentsPacket`
- `UpdateAltarReagentsPacket`
- ``vftable'`
- `operator=`
- `~UpdateAltarReagentsPacket` `V`

### `UpdateItemStackConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `UpdateItemStackConfigCmd`
- `UpdateItemStackConfigCmd`
- ``vftable'`
- `operator=`
- `~UpdateItemStackConfigCmd` `V`

### `UpdateItemStackConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UpdateItemStackConfigCmdPacket`
- `UpdateItemStackConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~UpdateItemStackConfigCmdPacket` `V`

### `UpdatePositionPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UpdatePositionPacket`
- `UpdatePositionPacket`
- ``vftable'`
- `operator=`
- `~UpdatePositionPacket` `V`

### `UseAction` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- `UseAction`
- `UseAction`
- ``vftable'`
- `~UseAction` `V`

### `UseItemConfigCmd` (Game.dll, 7)

- `Execute` `V`
- `GetNetPacket` `V`
- `UseItemConfigCmd`
- `UseItemConfigCmd`
- ``vftable'`
- `operator=`
- `~UseItemConfigCmd` `V`

### `UseItemConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UseItemConfigCmdPacket`
- `UseItemConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~UseItemConfigCmdPacket` `V`

### `UseItemOnConfigCmd` (Game.dll, 6)

- `Execute` `V`
- `GetNetPacket` `V`
- `UseItemOnConfigCmd`
- `UseItemOnConfigCmd`
- ``vftable'`
- `~UseItemOnConfigCmd` `V`

### `UseItemOnConfigCmdPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `UseItemOnConfigCmdPacket`
- `UseItemOnConfigCmdPacket`
- ``vftable'`
- `operator=`
- `~UseItemOnConfigCmdPacket` `V`

### `UseItemPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `UseItemPacket`
- `UseItemPacket`
- ``vftable'`
- `operator=`
- `~UseItemPacket` `V`

### `VideoPlayer` (Engine.dll, 13)

- `IsPlaying` `VC`
- `Pause` `V`
- `Play` `V`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `Render` `V`
- `Stop` `V`
- `Update` `V`
- `VideoPlayer`
- `VideoPlayer`
- ``vftable'`
- `operator=`
- `~VideoPlayer` `V`

### `ViewDistanceSectorData` (Engine.dll, 8)

- `Copy` `V`
- `ViewDistanceSectorData`
- `ViewDistanceSectorData`
- `ViewDistanceSectorData`
- ``vftable'`
- `operator=`
- `operator=`
- `~ViewDistanceSectorData` `V`

### `Viewport` (Engine.dll, 9)

- `ConvertFromDeviceCoords` `C`
- `GetAspectRatio` `C`
- `GetHeight` `C`
- `GetRect` `C`
- `GetWidth` `C`
- `Viewport`
- `Viewport`
- `operator=`
- `operator=`

### `VoiceChatManager` (Game.dll, 15)

- `GetClientState`
- `GetNextStreamSource` `C`
- `GetVoiceState` `C`
- `IsTransmitting` `C`
- `ProcessVoicePacket`
- `PushToTalk`
- `Update`
- `VoiceChatManager`
- `VoiceChatManager`
- `kDefaultBufferSize` `S`
- `kSampleRate` `S`
- `kTransmitBitRate` `S`
- `kTransmitComplexity` `S`
- `kTransmitReset` `S`
- `~VoiceChatManager`

### `VoiceChatPacket` (Engine.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `VoiceChatPacket`
- `VoiceChatPacket`
- ``vftable'`
- `operator=`
- `~VoiceChatPacket` `V`

### `WakeUpAction` (Game.dll, 8)

- `AnimationCallback` `V`
- `Execute` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- `WakeUpAction`
- `WakeUpAction`
- ``vftable'`
- `~WakeUpAction` `V`

### `WakeUpActionPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `WakeUpActionPacket`
- `WakeUpActionPacket`
- ``vftable'`
- `operator=`
- `~WakeUpActionPacket` `V`

### `WalkAction` (Game.dll, 8)

- `Execute` `V`
- `Finish` `V`
- `GetNetPacket` `V`
- `ToString` `VC`
- `WalkAction`
- `WalkAction`
- ``vftable'`
- `~WalkAction` `V`

### `WalkPacket` (Game.dll, 8)

- `Deserialize` `V`
- `GetPacketDescription` `V`
- `Serialize` `V`
- `WalkPacket`
- `WalkPacket`
- ``vftable'`
- `operator=`
- `~WalkPacket` `V`

### `WarpEntityPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `WarpEntityPacket`
- `WarpEntityPacket`
- ``vftable'`
- `operator=`
- `~WarpEntityPacket` `V`

### `Water` (Engine.dll, 40)

- `AddReflectionPlanes` `C`
- `AddToScene`
- `AddWaterCurve`
- `AddWaterLayer`
- `AddWaterPath`
- `AddWaveSet`
- `Cleanup`
- `CreatePhysics`
- `CreateRipple`
- `DestroyPhysics`
- `DrawCurveSelectionPoints` `C`
- `FindLayer`
- `GetBlockIndexBuffer` `C`
- `GetBlocksInFrustum` `C`
- `GetGrid` `C`
- `GetIntersection` `C`
- `GetLayer` `C`
- `GetNumLayers` `C`
- `GetRegion` `C`
- `GetTerrain` `C`
- `GetWaterPointInfo` `C`
- `GetWavesIndexBuffer` `C`
- `GetWavesShader` `C`
- `GetWavesStyle` `C`
- `Initialize`
- `IsWaterTypeUsed` `C`
- `Load`
- `LogInfo` `C`
- `Mirror`
- `PostDeviceReset`
- `PreDeviceReset`
- `RemoveWaterCurve`
- `RemoveWaterLayer`
- `RemoveWaterType`
- `Save`
- `SelectCurve` `C`
- `Update`
- `UpdateWaterType` `C`
- `Water`
- `~Water`

### `WaterCurve` (Engine.dll, 16)

- `DrawEditing` `V`
- `DrawSelectionPoints`
- `GetCurrentControlPoint`
- `GetCurveType` `C`
- `GetHeight` `C`
- `GetNumControlPoints` `C`
- `MirrorX` `V`
- `MirrorXZ` `V`
- `MirrorZ` `V`
- `SelectPoint`
- `UpdateCurrentControlPoint`
- `WaterCurve`
- `WaterCurve`
- `WaterCurve`
- ``vftable'`
- `~WaterCurve` `V`

### `WaterLayer` (Engine.dll, 38)

- `AddBlock`
- `CalcSurfaceVelocity` `C`
- `CalculateBlockBounds`
- `Cleanup`
- `ConstructBlockGeometry`
- `ConstructFromMap`
- `CreatePhysics`
- `DestroyBlockGeometry`
- `DestroyBlocks`
- `DestroyPhysics`
- `EditorUpdateArea`
- `GetBlock` `C`
- `GetHeight` `C`
- `GetNumBlocks` `C`
- `GetOffsetFromWorld`
- `GetReflectionPlane`
- `GetVertexBuffer` `C`
- `Initialize`
- `IsPointInLayer` `C`
- `Load` `V`
- `MirrorX` `V`
- `MirrorXZ` `V`
- `MirrorZ` `V`
- `PhysicsGetSurfaceType` `V`
- `PhysicsResponse` `V`
- `PhysicsSetup` `V`
- `RayIntersection` `C`
- `RemoveBlock`
- `Save` `VC`
- `SaveToMap`
- `TestBlockArea` `C`
- `Update` `V`
- `UpdateReflectionPlane`
- `WaterLayer`
- `WaterLayer`
- ``vftable'`
- ``vftable'`
- `~WaterLayer` `V`

### `WaterObject` (Engine.dll, 10)

- `GetIndex` `C`
- `GetRegion` `C`
- `GetWater` `C`
- `GetWaterType` `C`
- `GetWaterTypeName` `C`
- `ResetReflectionSurface`
- `WaterObject`
- `WaterObject`
- ``vftable'`
- `~WaterObject` `V`

### `WaterPath` (Engine.dll, 23)

- `AddControlPoint` `V`
- `CalcSurfaceVelocity` `C`
- `ConstructSegmentGeometry`
- `DestroySegmentGeometry`
- `DrawEditing` `V`
- `GetOffsetFromWorld`
- `GetReflectionPlane`
- `Initialize`
- `Load` `V`
- `PhysicsSetup` `V`
- `RayIntersection` `C`
- `RemoveEndControlPoint` `V`
- `Save` `VC`
- `SetWaterType`
- `Update` `V`
- `UpdateHeight` `V`
- `UpdateReflectionPlane`
- `UpdateSegment` `V`
- `WaterPath`
- `WaterPath`
- ``vftable'`
- ``vftable'`
- `~WaterPath` `V`

### `WaterRenderInterface` (Engine.dll, 13)

- `AddToScene`
- `Destroy`
- `GetDefaultReflectionTexture` `C`
- `GetFreeRenderBatch`
- `GetVertexDeclaration` `C`
- `GetWater` `C`
- `Initialize`
- `SetWater`
- `StortElementList` `C`
- `WaterRenderInterface`
- `WaterRenderInterface`
- `operator=`
- `~WaterRenderInterface`

### `WaterType` (Engine.dll, 74)

- `AddTexture`
- `BodySplashEffect`
- `BodySplashEffect` `C`
- `BodySplashTexture`
- `BodySplashTexture` `C`
- `CalculateOpacity` `C`
- `CalculateVelocity` `C`
- `ClearTextures`
- `Direction`
- `Direction` `C`
- `GetNumTextures` `C`
- `GetPhysicsSurface`
- `GetPhysicsSurface` `C`
- `GetShader` `C`
- `GetSplashTexture` `C`
- `GetTextureFile` `C`
- `GetTextureName` `C`
- `Load`
- `LoadResources`
- `MaxDepth`
- `MaxDepth` `C`
- `MaxDepthOpacity`
- `MaxDepthOpacity` `C`
- `MinDepth`
- `MinDepth` `C`
- `MinDepthOpacity`
- `MinDepthOpacity` `C`
- `NoiseTextureName`
- `NoiseTextureName` `C`
- `ObjectSplashEffect`
- `ObjectSplashEffect` `C`
- `ObjectSplashTexture`
- `ObjectSplashTexture` `C`
- `Reflectivity`
- `Reflectivity` `C`
- `ReloadNoise`
- `ReloadShader`
- `ReloadTextures`
- `RemoveTexture`
- `Save` `C`
- `ScrollSpeed`
- `ScrollSpeed` `C`
- `SetDefaultValues`
- `SetShaderParams` `C`
- `SetTexture`
- `ShaderName`
- `ShaderName` `C`
- `Smoothness`
- `Smoothness` `C`
- `Specularity`
- `Specularity` `C`
- `TextureScale`
- `TextureScale` `C`
- `TypeName`
- `TypeName` `C`
- `WakeSplashEffect`
- `WakeSplashEffect` `C`
- `WakeSplashTexture`
- `WakeSplashTexture` `C`
- `WaterColor`
- `WaterColor` `C`
- `WaterType`
- `WaterType`
- `WaterType`
- `kDefaultBodyTexture` `S`
- `kDefaultNoiseTexture` `S`
- `kDefaultObjectTexture` `S`
- `kDefaultShaderName` `S`
- `kDefaultSkyMap` `S`
- `kDefaultSplashEffect` `S`
- `kDefaultWakeTexture` `S`
- `kVersion` `S`
- `operator=`
- `~WaterType`

### `WaterTypeManager` (Engine.dll, 15)

- `AddWaterType`
- `AddWaterTypeReference`
- `CreateWaterType`
- `Destroy` `S`
- `FindWaterType`
- `Get` `S`
- `GetWaterType`
- `GetWaterTypeNames` `C`
- `IsWaterType`
- `ReleaseWaterType`
- `RenameWaterType`
- `WaterTypeManager`
- `WaterTypeManager`
- `instance` `S`
- `~WaterTypeManager`

### `WaterWaveSet` (Engine.dll, 17)

- `AddControlPoint` `V`
- `ConstructSegmentGeometry`
- `DestroySegmentGeometry`
- `GetWaveParams`
- `Initialize`
- `Load` `V`
- `RayIntersection` `C`
- `RemoveEndControlPoint` `V`
- `Save` `VC`
- `Update` `V`
- `UpdateHeight` `V`
- `UpdateSegment` `V`
- `UpdateWaves`
- `WaterWaveSet`
- `WaterWaveSet`
- ``vftable'`
- `~WaterWaveSet` `V`

### `Weapon` (Game.dll, 46)

- `AttachItem` `V`
- `ClearWeaponEnchantment`
- `CreateUIAttributeText` `VC`
- `DetachItem` `V`
- `DisableWeaponTrail` `V`
- `EnableWeaponTrail` `V`
- `GetAttackAttachpoint` `VC`
- `GetDefaultAttackEffectName` `C`
- `GetDefaultProjectileName` `C`
- `GetDefaultWeaponTrailName` `C`
- `GetGameDescription` `VC`
- `GetHitSound` `C`
- `GetProjectileName` `C`
- `GetProjectilePiercingChance` `C`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetSwipeSound` `C`
- `GetUIGameDescription` `VC`
- `GetUIQualityDescription` `VC`
- `GetWeaponEnchantmentName` `C`
- `GetWeaponType` `VC`
- `GetWeaponTypeTag` `S`
- `IsRanged` `C`
- `IsTwoHanded` `C`
- `IsTwoHandedMeleeWeapon` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PlayHitSound` `C`
- `PlaySwipeSound` `C`
- `PreLoad` `V`
- `RTTI_new` `S`
- `RemoveAttachments` `V`
- `RemoveAttackEffect`
- `ResetAttackEffect`
- `ResetWeaponTrail`
- `SetOverrideAttackEffect`
- `SetWeaponEnchantment` `V`
- `SwitchWeaponTrail` `V`
- `UpdateSelf` `V`
- `Weapon`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~Weapon` `V`

### `WeaponArmor` (Game.dll, 13)

- `CreateUIAttributeText` `VC`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetUIArmorSpecificText` `VC`
- `RTTI_new` `S`
- `WeaponArmor`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponArmor` `V`

### `WeaponArmor_Offhand` (Game.dll, 16)

- `AttachItem` `V`
- `DetachItem` `V`
- `GetItemMarketType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponArmor_Offhand`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponArmor_Offhand` `V`

### `WeaponArmor_Shield` (Game.dll, 26)

- `AttachItem` `V`
- `DetachItem` `V`
- `GetItemMarketType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetShieldAbsorption` `C`
- `GetShieldBlockChance` `C`
- `GetShieldDefense` `C`
- `GetShieldRecoveryTime` `C`
- `GetStaticClassInfo` `S`
- `GetUIArmorSpecificText` `VC`
- `GetWeaponType` `VC`
- `Load` `V`
- `OnDestroy` `V`
- `PlayBlockEffect`
- `PlayBlockSound`
- `PreLoad` `V`
- `RTTI_new` `S`
- `WeaponArmor_Shield`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponArmor_Shield` `V`

### `WeaponHunting` (Game.dll, 11)

- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `WeaponHunting`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponHunting` `V`

### `WeaponHunting_Ranged1h` (Game.dll, 15)

- `GetAttackAttachpoint` `VC`
- `GetItemMarketType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponHunting_Ranged1h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponHunting_Ranged1h` `V`

### `WeaponHunting_Ranged2h` (Game.dll, 15)

- `GetAttackAttachpoint` `VC`
- `GetItemMarketType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponHunting_Ranged2h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponHunting_Ranged2h` `V`

### `WeaponHunting_Spear` (Game.dll, 14)

- `GetItemMarketType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponHunting_Spear`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponHunting_Spear` `V`

### `WeaponMagical` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetItemType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `WeaponMagical`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMagical` `V`

### `WeaponMagical_Staff` (Game.dll, 13)

- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMagical_Staff`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMagical_Staff` `V`

### `WeaponMelee` (Game.dll, 13)

- `GetItemType` `VC`
- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `WeaponMelee`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee` `V`

### `WeaponMelee_Axe` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Axe`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Axe` `V`

### `WeaponMelee_Axe2h` (Game.dll, 13)

- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Axe2h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Axe2h` `V`

### `WeaponMelee_Dagger` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Dagger`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Dagger` `V`

### `WeaponMelee_Mace` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Mace`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Mace` `V`

### `WeaponMelee_Mace2h` (Game.dll, 13)

- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Mace2h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Mace2h` `V`

### `WeaponMelee_Scepter` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Scepter`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Scepter` `V`

### `WeaponMelee_Spear2h` (Game.dll, 13)

- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Spear2h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Spear2h` `V`

### `WeaponMelee_Sword` (Game.dll, 12)

- `GetItemMarketType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Sword`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Sword` `V`

### `WeaponMelee_Sword2h` (Game.dll, 13)

- `GetLeftHandType` `VC`
- `GetRTTIClassInfo` `VC`
- `GetRightHandType` `VC`
- `GetStaticClassInfo` `S`
- `GetWeaponType` `VC`
- `RTTI_new` `S`
- `WeaponMelee_Sword2h`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponMelee_Sword2h` `V`

### `WeaponTrail` (Engine.dll, 13)

- `GetAnchorPoints`
- `GetRTTIClassInfo` `VC`
- `GetStaticClassInfo` `S`
- `RTTI_new` `S`
- `SetPointSetIndex`
- `UpdateSelf` `V`
- `WeaponTrail`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- ``vftable'`
- `classInfo` `S`
- `~WeaponTrail` `V`

### `WeatherManager` (Engine.dll, 46)

- `AddToScene`
- `ChangeClimate`
- `CreateSharedResources`
- `DestroySharedResources`
- `FindSystem`
- `GetBounds` `C`
- `GetBoundsInRegion` `C`
- `GetClimateChoice` `C`
- `GetMovementAdjustment` `C`
- `GetNormalizedWindStrength` `C`
- `GetRandomGen`
- `GetRegion` `C`
- `GetShader` `C`
- `GetSound` `C`
- `GetSpawnDeltaMultiplier` `C`
- `GetTemplate`
- `GetTexture` `C`
- `GetWind` `C`
- `GetWindDirection` `C`
- `GetWindStrength` `C`
- `Initialize`
- `IsEnabled` `C`
- `PostDeviceReset` `V`
- `PreDeviceReset` `V`
- `RandomizeWind`
- `RefreshIntensities`
- `RegisterParticleModifier`
- `Reset`
- `SelectClimateChoice`
- `SetDebug`
- `SetEnabled`
- `SetStates`
- `SetWeather`
- `Shutdown`
- `StopAllSystems`
- `Update`
- `UpdateAdvancedWind`
- `UpdateAmbience`
- `UpdateModifiers`
- `UpdateWeatherSystemsForCurrentClimate`
- `WeatherManager`
- `WeatherManager`
- ``vftable'`
- `maxParticles` `S`
- `operator=`
- `~WeatherManager` `V`

### `Widget` (Widget.dll, 39)

- `Attach`
- `BitBlt` `C`
- `Center`
- `ContainsCursor` `C`
- `EnableSubClassing`
- `GetHandle` `C`
- `GetHeight` `C`
- `GetText` `C`
- `GetWidth` `C`
- `OnChildEvent` `V`
- `OnCreate` `V`
- `OnDestroy` `V`
- `OnHScroll` `V`
- `OnIdle` `V`
- `OnKeyDown` `V`
- `OnKeyUp` `V`
- `OnMenuSelection` `V`
- `OnMouseButton` `V`
- `OnMouseMove` `V`
- `OnMouseWheel` `V`
- `OnNotify` `V`
- `OnPaint` `V`
- `OnSizeChange` `V`
- `OnTimer` `V`
- `OnUpdateMenu` `V`
- `OnVScroll` `V`
- `Redraw`
- `SendNotification`
- `SetEnabled`
- `SetExStyle`
- `SetFont`
- `SetFontToDefault`
- `SetPosition`
- `SetSize`
- `SetStyle`
- `SetText`
- `Show`
- `Widget`
- `~Widget` `V`

### `WinWindow` (Engine.dll, 38)

- `Activate` `V`
- `Center` `V`
- `Close` `V`
- `Destroy`
- `DisableSetCursor` `V`
- `GetClientHeight` `VC`
- `GetClientWidth` `VC`
- `GetHeight` `VC`
- `GetMonitorRect` `V`
- `GetStyle` `VC`
- `GetSystemWindow` `VC`
- `GetWidth` `VC`
- `GetX` `VC`
- `GetY` `VC`
- `Initialize` `V`
- `IsActive` `V`
- `Maximize` `V`
- `Minimize` `V`
- `OnActivate`
- `OnMinimize`
- `OnRestore`
- `OnToggleFullscreen`
- `ProcessMessages` `V`
- `RegisterEventHandler` `V`
- `ScreenSaverWindowProc` `S`
- `SetCaption` `V`
- `SetCursor` `V`
- `SetGammaRamp` `V`
- `SetSize` `V`
- `SetTopmost` `V`
- `Show` `V`
- `UnregisterEventHandler` `V`
- `WinWindow`
- `WinWindow`
- `WindowProc` `S`
- ``vftable'`
- `operator=`
- `~WinWindow` `V`

### `Window` (Widget.dll, 67)

- `BeginDoubleBufferedPainting`
- `CaptureMouse`
- `Center`
- `CheckMenuItem`
- `Create`
- `Create` `V`
- `DefaultMessageHandle` `V`
- `Destroy`
- `EnableChildEnumProc` `S`
- `EndDoubleBufferedPainting`
- `GetChildValue` `C`
- `GetChildWidget` `C`
- `GetClientHeight` `C`
- `GetClientWidth` `C`
- `GetHandle` `C`
- `GetMenu` `C`
- `GetParent`
- `HandleMessage` `S`
- `HandleMouseButtonEvent`
- `IsMenuItemChecked` `C`
- `IsVisible` `C`
- `Maximize`
- `MessageBoxA` `C`
- `Minimize`
- `OnChildEvent` `V`
- `OnClose` `V`
- `OnCreate` `V`
- `OnDestroy` `V`
- `OnDoubleClick` `V`
- `OnDrawItem` `V`
- `OnEnable` `V`
- `OnHScroll` `V`
- `OnHitTest` `V`
- `OnIdle` `V`
- `OnKeyDown` `V`
- `OnKeyUp` `V`
- `OnMenuSelection` `V`
- `OnMouseButton` `V`
- `OnMouseMove` `V`
- `OnMouseWheel` `V`
- `OnMoved` `V`
- `OnNotify` `V`
- `OnPaint` `V`
- `OnSetCursor` `V`
- `OnShow` `V`
- `OnSizeChange` `V`
- `OnSizing` `V`
- `OnTimer` `V`
- `OnUpdateMenu` `V`
- `OnUserMessage` `V`
- `OnVScroll` `V`
- `ReleaseMouse`
- `RunModal`
- `SetAccelerator`
- `SetBackground`
- `SetCaption`
- `SetChildrenEnabled`
- `SetForwardCommandsToParent`
- `SetIcon`
- `SetMenu`
- `SetMenu`
- `Show`
- `StartTimer`
- `StopModal`
- `StopTimer`
- `UpdateMenuItems`
- `Window`

### `WindowEventHandler` (Engine.dll, 10)

- `OnActivate` `V`
- `OnMinimize` `V`
- `OnRestore` `V`
- `OnToggleFullscreen` `V`
- `WindowEventHandler`
- `WindowEventHandler`
- `WindowEventHandler`
- ``vftable'`
- `operator=`
- `operator=`

### `World` (Engine.dll, 110)

- `AddEntity`
- `AddEntityToNetworkList`
- `AddFOWVisibility`
- `AddRegion`
- `AddRegion`
- `AddRegion`
- `AreRegionsConnected` `C`
- `BuildConnectivity`
- `CheckLOS` `C`
- `CreateDirectionalDisturbance`
- `CreateDirectionalWind`
- `CreateLineDisturbance`
- `CreatePointDisturbance`
- `CreateWorldGrid`
- `DestroyAllEntities`
- `EnableDebugging`
- `GeometryBusStop`
- `GeometryBusStop`
- `GetAllIntersections` `C`
- `GetConnectedRegionSet`
- `GetDirection` `C`
- `GetDistance` `C`
- `GetDistanceSquared` `C`
- `GetEntities` `C`
- `GetEntities` `C`
- `GetEntitiesAroundRay` `C`
- `GetEntitiesInBox` `C`
- `GetEntitiesInCone`
- `GetEntitiesInFrustum` `C`
- `GetEntitiesInSphere` `C`
- `GetFOWManager`
- `GetFileName` `C`
- `GetHighestTerrainHeight`
- `GetIntersection` `C`
- `GetIntersectionThroughPortals` `C`
- `GetLevelRegion` `C`
- `GetLoadedRegionsInFrustum` `C`
- `GetNearestFloorHeight` `C`
- `GetNumLevelsLoaded` `C`
- `GetNumLevelsLoading` `C`
- `GetNumQuestFiles` `C`
- `GetNumRegions` `C`
- `GetPlayerSpawnPoint` `C`
- `GetQuestFile` `C`
- `GetRegion`
- `GetRegionById`
- `GetRegionContainingPoint` `C`
- `GetRegionContainingPoint` `C`
- `GetRegionContainingXZ` `C`
- `GetRegionIndex` `C`
- `GetRegionIndex` `C`
- `GetRegionLevel` `C`
- `GetRegionName` `C`
- `GetRegionsInBox` `C`
- `GetRegionsInFrustum` `C`
- `GetRegionsInSphere` `C`
- `GetShadowSoftness` `C`
- `GetSurfaceTypeBelow` `C`
- `GetTQ1MapFileVersionNumber` `C`
- `GuaranteedGetRegionLevel` `C`
- `InstallMapChunkCallback`
- `InvalidateAllPathing`
- `IsInFog` `C`
- `IsValidMapFile`
- `IsValidWorldFile`
- `Load`
- `LoadMap`
- `LockMapFile` `C`
- `LogRegionStates` `C`
- `PickEntities` `C`
- `PickEntity` `C`
- `PickEntity` `C`
- `PickRegion` `C`
- `PlaceDecal`
- `PlaceDecalOnGround`
- `PostDeviceReset`
- `PostLoadPass`
- `PreDeviceReset`
- `PutOnFloor`
- `RegionsLoaded` `C`
- `ReleaseMinimapImages`
- `ReloadGenerationDependentData`
- `RemoveAllQuestFiles`
- `RemoveEntity`
- `RemoveEntityFromNetworkList`
- `RemoveRegion`
- `Save`
- `SetCoords`
- `SetFileName`
- `SetNumQuestFiles`
- `SetPlayerSpawnPoint`
- `SetQuestFile`
- `SetRegionLoaded`
- `SetRegionName`
- `SetShadowSoftness`
- `ShiftFrustum` `C`
- `ShowConnections`
- `TraceRayAgainstPortals` `C`
- `Unload`
- `UnloadAllRegions`
- `UnloadExtraRegions`
- `UnloadFOWForMapClose`
- `UnloadLevel`
- `UnlockMapFile` `C`
- `Update`
- `UpdateRegionBoundingBox`
- `UpdateRegionUsage`
- `ValidateSectorLayers`
- `World`
- `~World` `V`

### `WorldABBox` (Engine.dll, 9)

- `Contains` `C`
- `GetIntersection` `C`
- `GetMax` `C`
- `GetMin` `C`
- `GetRegionBox` `C`
- `WorldABBox`
- `WorldABBox`
- `operator=`
- `operator=`

### `WorldCamera` (Engine.dll, 40)

- `CalculateViewPosition` `VC`
- `GetCameraDistance` `C`
- `GetCameraDistanceDefault` `C`
- `GetCameraFOV` `C`
- `GetCameraFarPlane` `C`
- `GetCameraNearPlane` `C`
- `GetCameraOffset`
- `GetCameraOffset` `C`
- `GetCameraPitch` `C`
- `GetCameraPitchDefault` `C`
- `GetCameraYaw` `C`
- `GetCoords` `C`
- `GetDistance` `C`
- `GetFrustum` `C`
- `GetFrustum` `C`
- `GetImagePoint` `C`
- `GetOffsetFromTarget` `C`
- `GetRayThroughImagePoint` `C`
- `GetRegion` `C`
- `GetRegionCamera` `C`
- `GetRegionFrustum` `C`
- `GetRegionFrustum` `C`
- `GetSubFrustum` `C`
- `GetSubFrustum` `C`
- `GetZoomTimer`
- `Project` `C`
- `SetAcceleration`
- `SetCameraDistance` `V`
- `SetCameraDistanceDefault`
- `SetCameraFOV`
- `SetCameraFarPlane`
- `SetCameraNearPlane`
- `SetCameraPitch`
- `SetCameraPitchDefault`
- `SetCameraYaw` `V`
- `SetListenerPosition`
- `SetRegionCamera`
- `Update` `V`
- `UpdateFromInput`
- `WorldCamera`

### `WorldCoords` (Engine.dll, 20)

- `ClearRotation`
- `GetRegionCoords` `C`
- `LookAt` `S`
- `LookAtNoLean` `S`
- `LookIn` `S`
- `Orthonormalize`
- `Read`
- `RotateAroundPoint` `S`
- `SetOrigin`
- `TransformVector` `C`
- `Translate`
- `Translation` `S`
- `WorldCoords`
- `WorldCoords`
- `WorldCoords`
- `WorldCoords`
- `Write`
- `Write`
- `operator=`
- `operator=`

### `WorldFile` (Engine.dll, 13)

- `GetIconData` `C`
- `GetIconDataSize` `C`
- `GetInstanceData` `C`
- `GetInstanceDataSize` `C`
- `GetNumQuestFiles` `C`
- `GetNumRegions` `C`
- `GetQuestFile` `C`
- `GetRegion` `C`
- `Read`
- `WorldFile`
- `WorldFile`
- `operator=`
- `~WorldFile`

### `WorldFrustum` (Engine.dll, 9)

- `GetRegion` `C`
- `GetRegionFrustum`
- `GetRegionFrustum` `C`
- `GetRelativeFrustum` `C`
- `Initialize`
- `WorldFrustum`
- `WorldFrustum`
- `operator=`
- `operator=`

### `WorldInitPacket` (Game.dll, 8)

- `CopyInbound` `V`
- `GetPacketDescription` `V`
- `PrepareOutBuffer` `V`
- `WorldInitPacket`
- `WorldInitPacket`
- ``vftable'`
- `operator=`
- `~WorldInitPacket` `V`

### `WorldIntersection` (Engine.dll, 3)

- `WorldIntersection`
- `operator=`
- `operator=`

### `WorldVec3` (Engine.dll, 18)

- `GetRegion` `C`
- `GetRegionPosition` `C`
- `GetWorldPosition` `C`
- `Length` `C`
- `MakeRelative`
- `PutOnFloor`
- `Read`
- `SetFromWorldPosition`
- `Translate`
- `TranslateInRegion`
- `TranslateToFloor`
- `WorldVec3`
- `WorldVec3`
- `Write`
- `Write`
- `operator-` `C`
- `operator=`
- `operator=`

### `ZoneManager` (Engine.dll, 14)

- `Destroy` `S`
- `Get` `S`
- `GetActFromRegion` `C`
- `GetZoneData` `C`
- `GetZoneList` `C`
- `LoadDBR`
- `RenderKey` `C`
- `RenderZoneKey` `C`
- `ZoneManager`
- `ZoneManager`
- ``vftable'`
- `instance` `S`
- `operator=`
- `~ZoneManager` `V`

### ``2'` (Engine.dll, 1)

- `checksumTable`

### ``2'` (Game.dll, 2)

- `end`
- `nEnd`

### `<free functions>` (Engine.dll, 24)

- `LZ4_compressBound`
- `LZ4_compress_default`
- `LZ4_compress_destSize`
- `LZ4_compress_fast`
- `LZ4_compress_fast_continue`
- `LZ4_compress_fast_extState`
- `LZ4_createStream`
- `LZ4_createStreamDecode`
- `LZ4_decompress_fast`
- `LZ4_decompress_fast_continue`
- `LZ4_decompress_fast_usingDict`
- `LZ4_decompress_safe`
- `LZ4_decompress_safe_continue`
- `LZ4_decompress_safe_partial`
- `LZ4_decompress_safe_usingDict`
- `LZ4_freeStream`
- `LZ4_freeStreamDecode`
- `LZ4_loadDict`
- `LZ4_resetStream`
- `LZ4_saveDict`
- `LZ4_setStreamDecode`
- `LZ4_sizeofState`
- `LZ4_versionNumber`
- `LZ4_versionString`

### `<free functions>` (Game.dll, 4)

- `InitializeObjects`
- `ShutdownObjects`
- `TSS0<`template-parameter-2',GAME::ControllerMegalesiosStateLaunchBursts::ndleEvent,GAME::EAAXAEBVName>`
- `TSS0<`template-parameter-2',GAME::ControllerOrmenosStateAttack::ndleEvent,float,unsigned char,void &,GAME::Name const & __ptr64>`

### `<free functions>` (Widget.dll, 24)

- `LZ4_compressBound`
- `LZ4_compress_default`
- `LZ4_compress_destSize`
- `LZ4_compress_fast`
- `LZ4_compress_fast_continue`
- `LZ4_compress_fast_extState`
- `LZ4_createStream`
- `LZ4_createStreamDecode`
- `LZ4_decompress_fast`
- `LZ4_decompress_fast_continue`
- `LZ4_decompress_fast_usingDict`
- `LZ4_decompress_safe`
- `LZ4_decompress_safe_continue`
- `LZ4_decompress_safe_partial`
- `LZ4_decompress_safe_usingDict`
- `LZ4_freeStream`
- `LZ4_freeStreamDecode`
- `LZ4_loadDict`
- `LZ4_resetStream`
- `LZ4_saveDict`
- `LZ4_setStreamDecode`
- `LZ4_sizeofState`
- `LZ4_versionNumber`
- `LZ4_versionString`
