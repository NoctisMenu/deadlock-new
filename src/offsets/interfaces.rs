// Generated using https://github.com/a2x/cs2-dumper
// 2026-07-04 03:38:29.278121500 UTC

#![allow(non_upper_case_globals, unused)]

pub mod cs2_dumper {
    pub mod interfaces {
        // Module: animationsystem.dll
        pub mod animationsystem_dll {
            pub const AnimationSystemUtils_001: usize = 0x7E5770;
            pub const AnimationSystem_001: usize = 0x7DD690;
        }
        // Module: client.dll
        pub mod client_dll {
            pub const ClientToolsInfo_001: usize = 0x2EAFA70;
            pub const GameClientExports001: usize = 0x2EAC750;
            pub const Source2Client002: usize = 0x378E020;
            pub const Source2ClientConfig001: usize = 0x3282AD0;
            pub const Source2ClientPrediction001: usize = 0x2EB6D20;
            pub const Source2ClientUI001: usize = 0x2EC64A0;
        }
        // Module: engine2.dll
        pub mod engine2_dll {
            pub const BenchmarkService001: usize = 0x61F980;
            pub const BugService001: usize = 0x8D61C0;
            pub const ClientServerEngineLoopService_001: usize = 0x917840;
            pub const EngineGameUI001: usize = 0x61D310;
            pub const EngineServiceMgr001: usize = 0x917120;
            pub const GameEventSystemClientV001: usize = 0x917400;
            pub const GameEventSystemServerV001: usize = 0x917530;
            pub const GameResourceServiceClientV001: usize = 0x61FA80;
            pub const GameResourceServiceServerV001: usize = 0x61FAE0;
            pub const GameUIService_001: usize = 0x8D65F0;
            pub const HostStateMgr001: usize = 0x620300;
            pub const INETSUPPORT_001: usize = 0x618920;
            pub const InputService_001: usize = 0x8D68E0;
            pub const KeyValueCache001: usize = 0x6203B0;
            pub const MapListService_001: usize = 0x915750;
            pub const NetworkClientService_001: usize = 0x9158E0;
            pub const NetworkP2PService_001: usize = 0x915C20;
            pub const NetworkServerService_001: usize = 0x915DD0;
            pub const NetworkService_001: usize = 0x61FC50;
            pub const RenderService_001: usize = 0x916040;
            pub const ScreenshotService001: usize = 0x916300;
            pub const SimpleEngineLoopService_001: usize = 0x620410;
            pub const SoundService_001: usize = 0x61FC90;
            pub const Source2EngineToClient001: usize = 0x61CC30;
            pub const Source2EngineToClientStringTable001: usize = 0x61CC90;
            pub const Source2EngineToServer001: usize = 0x61CD08;
            pub const Source2EngineToServerStringTable001: usize = 0x61CD30;
            pub const SplitScreenService_001: usize = 0x61FF70;
            pub const StatsService_001: usize = 0x9166C0;
            pub const ToolService_001: usize = 0x620130;
            pub const VENGINE_GAMEUIFUNCS_VERSION005: usize = 0x61D3A0;
            pub const VProfService_001: usize = 0x620170;
        }
        // Module: filesystem_stdio.dll
        pub mod filesystem_stdio_dll {
            pub const VAsyncFileSystem2_001: usize = 0x2149E0;
            pub const VFileSystem017: usize = 0x2147A0;
        }
        // Module: host.dll
        pub mod host_dll {
            pub const DebugDrawQueueManager001: usize = 0x139000;
            pub const GameModelInfo001: usize = 0x139040;
            pub const GameSystem2HostHook: usize = 0x139080;
            pub const HostUtils001: usize = 0x1466C0;
            pub const PredictionDiffManager001: usize = 0x139190;
            pub const SaveRestoreDataVersion001: usize = 0x1392C0;
            pub const SinglePlayerSharedMemory001: usize = 0x1392F0;
            pub const Source2Host001: usize = 0x139360;
        }
        // Module: imemanager.dll
        pub mod imemanager_dll {
            pub const IMEManager001: usize = 0x36B20;
        }
        // Module: inputsystem.dll
        pub mod inputsystem_dll {
            pub const InputStackSystemVersion001: usize = 0x40E30;
            pub const InputSystemVersion001: usize = 0x42B50;
        }
        // Module: localize.dll
        pub mod localize_dll {
            pub const Localize_001: usize = 0x56EA0;
        }
        // Module: materialsystem2.dll
        pub mod materialsystem2_dll {
            pub const FontManager_001: usize = 0x1646A0;
            pub const MaterialUtils_001: usize = 0x14C570;
            pub const PostProcessingSystem_001: usize = 0x14C480;
            pub const TextLayout_001: usize = 0x14C500;
            pub const VMaterialSystem2_001: usize = 0x163F90;
        }
        // Module: meshsystem.dll
        pub mod meshsystem_dll {
            pub const MeshSystem001: usize = 0x14A720;
        }
        // Module: navsystem.dll
        pub mod navsystem_dll {
            pub const NavSystem001: usize = 0x1219F0;
        }
        // Module: networksystem.dll
        pub mod networksystem_dll {
            pub const FlattenedSerializersVersion001: usize = 0x273780;
            pub const NetworkMessagesVersion001: usize = 0x29B800;
            pub const NetworkSystemVersion001: usize = 0x28CF30;
            pub const SerializedEntitiesVersion001: usize = 0x28D020;
        }
        // Module: panorama.dll
        pub mod panorama_dll {
            pub const PanoramaUIEngine001: usize = 0x508D30;
        }
        // Module: panorama_text_pango.dll
        pub mod panorama_text_pango_dll {
            pub const PanoramaTextServices001: usize = 0x2B8A40;
        }
        // Module: panoramauiclient.dll
        pub mod panoramauiclient_dll {
            pub const PanoramaUIClient001: usize = 0x2933A0;
        }
        // Module: particles.dll
        pub mod particles_dll {
            pub const ParticleSystemMgr003: usize = 0x519950;
        }
        // Module: pulse_system.dll
        pub mod pulse_system_dll {
            pub const IPulseSystem_001: usize = 0x1F0A40;
        }
        // Module: rendersystemdx11.dll
        pub mod rendersystemdx11_dll {
            pub const RenderDeviceMgr001: usize = 0x430DB0;
            pub const RenderUtils_001: usize = 0x4316A8;
            pub const VRenderDeviceMgrBackdoor001: usize = 0x430E50;
        }
        // Module: resourcesystem.dll
        pub mod resourcesystem_dll {
            pub const ResourceSystem013: usize = 0x83000;
        }
        // Module: scenefilecache.dll
        pub mod scenefilecache_dll {
            pub const ResponseRulesCache001: usize = 0xE17F0;
            pub const SceneFileCache002: usize = 0xE1978;
        }
        // Module: scenesystem.dll
        pub mod scenesystem_dll {
            pub const RenderingPipelines_001: usize = 0x658B30;
            pub const SceneSystem_002: usize = 0x8CD2A0;
            pub const SceneUtils_001: usize = 0x659A40;
        }
        // Module: schemasystem.dll
        pub mod schemasystem_dll {
            pub const SchemaSystem_001: usize = 0x767E0;
        }
        // Module: server.dll
        pub mod server_dll {
            pub const EntitySubclassUtilsV001: usize = 0x2FF6F40;
            pub const NavGameTest001: usize = 0x3145BD8;
            pub const ServerToolsInfo_001: usize = 0x30F3B68;
            pub const Source2GameClients001: usize = 0x30EF1B0;
            pub const Source2GameDirector001: usize = 0x34D5B00;
            pub const Source2GameEntities001: usize = 0x30F3260;
            pub const Source2Server001: usize = 0x30F30D0;
            pub const Source2ServerConfig001: usize = 0x35280B8;
        }
        // Module: soundsystem.dll
        pub mod soundsystem_dll {
            pub const SoundOpSystem001: usize = 0x504D80;
            pub const SoundOpSystemEdit001: usize = 0x504C40;
            pub const SoundSystem001: usize = 0x504730;
            pub const VMixEditTool001: usize = 0x59476BF;
        }
        // Module: steamaudio.dll
        pub mod steamaudio_dll {
            pub const SteamAudio001: usize = 0x25D600;
        }
        // Module: tier0.dll
        pub mod tier0_dll {
            pub const TestScriptMgr001: usize = 0x39B710;
            pub const VEngineCvar007: usize = 0x3A6530;
            pub const VProcessUtils002: usize = 0x39B6B0;
            pub const VStringTokenSystem001: usize = 0x3CD220;
        }
        // Module: v8system.dll
        pub mod v8system_dll {
            pub const Source2V8System001: usize = 0x31730;
        }
        // Module: vphysics2.dll
        pub mod vphysics2_dll {
            pub const VPhysics2_Handle_Interface_001: usize = 0x402210;
            pub const VPhysics2_Interface_001: usize = 0x402250;
        }
        // Module: vscript.dll
        pub mod vscript_dll {
            pub const VScriptManager010: usize = 0x13B410;
        }
        // Module: worldrenderer.dll
        pub mod worldrenderer_dll {
            pub const WorldRendererMgr001: usize = 0x221FE0;
        }
    }
}
