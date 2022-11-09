# Developers

[Back to Home](/README.md#table-of-contents)

**DISCLAMER!!!** This section is designed to provide information for experienced developers! If you are just looking to play games or customize your deck, we recommend returning ot the main page and ignoring this section!

## Table of Contents

- []()

## Engines

## Steam Deck Development

[Back to the Top](#developers)

### Quick Links to Development Resources

[Back to the Top](#developers)

- [Getting your game ready for Steam Deck](https://partner.steamgames.com/doc/steamdeck/recommendations)

- [Developing for Steam Deck without a Dev-Kit](https://partner.steamgames.com/doc/steamdeck/testing)

- [Steam Deck Developer Kits](https://partner.steamgames.com/doc/steamdeck/devkits)

- [SteamOS Devkit Service on GitLab](https://gitlab.steamos.cloud/devkit/steamos-devkit-service)

- [SteamOS Devkit Client on GitLab](https://gitlab.steamos.cloud/devkit/steamos-devkit)

- [Steam Deck and Proton](https://partner.steamgames.com/doc/steamdeck/proton)

- [Steam Deck Developer Forums](https://steamcommunity.com/groups/steamworks/discussions/27/)

[Steamworks](https://partner.steamgames.com/doc/home) is a free suite of tools available to any developer to use in their game or software on Steam and the Steam Deck.

[Steam Hardware GitLab Repo](https://gitlab.steamos.cloud/SteamDeck/hardware) is a repository that contains CAD files for the external shell (surface topology) of Steam Deck, under a Creative Commons license. This includes an STP model, STL model, and drawings (DWG) for reference.

[Dynamic Cloud Sync](https://steamcommunity.com/groups/steamworks/announcements/detail/3142949576401813670) is a tool that Steam will use  to automatically upload all modified save game data to the cloud prior to the device entering sleep mode. Users can then resume their game on any PC, laptop or other device. Steam will also automatically download any save game changes when users return to their Steam Deck and wake up the device.

[Steam Cloud](https://partner.steamgames.com/doc/features/cloud) is a tool that provides an easy and transparent remote file storage system for your game. Files specified in the Auto-Cloud configuration or written to disk (created, modified, deleted, etc.) using the Cloud API will automatically be replicated to the Steam servers after the game exits. If the user changes computers, the files are automatically downloaded to the new computer prior to the game launching. The game can then access the files by reading them through the Cloud API or reading them directly from disk as usual. Avoid machine specific configurations such as video settings.

[Gamescope](https://github.com/Plagman/gamescope) is a SteamOS session micro-compositing window manager formerly known as [steamcompmgr](https://github.com/ValveSoftware/steamos-compositor).

[AMD FidelityFX Super Resolution (FSR)](https://www.amd.com/en/technologies/radeon-software-fidelityfx) is an open source, high-quality solution for producing high resolution frames from lower resolution inputs. FSR enables “practical performance” for costly render operations, such as hardware ray tracing for the AMD RDNA™ and AMD RDNA™ 2 architectures.

[AMD FidelityFX Super Resolution (FSR) 2.0](https://github.com/GPUOpen-Effects/FidelityFX-FSR2) is an open source, high-quality solution for producing high resolution frames from lower resolution inputs. It uses temporal data and optimized anti-aliasing to boost framerates in supported games while delivering similar or better image quality than native resolution without requiring dedicated machine learning hardware.

[AMD FidelityFX Super Resolution 3 (FSR 3)](https://www.amd.com/en/technologies/fidelityfx-super-resolution), is the latest version of the company’s upscaling tech. AMD claims it’ll provide 2x times increase in frame rate over FSR 2, boosted by the new AMD Fluid Motion Frames technology.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/200111466-0469f86c-2450-4ae6-8f09-4c578c6b630d.png">
<br />
</p>

[AMD Precision Boost Overdrive 2 (PBO2)](https://www.amd.com/en/support/kb/faq/cpu-pb2) is a performance-maximizing technology available will work with the Ryzen 5000 or newer AMD processors improving your PC's performance by raising clockspeeds, which makes the processor and your applications run faster. This technology will not be applied retroactively to previous-gen models, like Ryzen 3000 chips or newer.

[MangoHud](https://github.com/flightlessmango/MangoHud) is a Vulkan and OpenGL overlay for monitoring FPS, temperatures, CPU/GPU load and more.

[GOverlay](https://github.com/benjamimgois/goverlay) is an open source project aimed to create a Graphical UI to manage Vulkan/OpenGL overlays. It is still in early development.

[ReplaySorcery](https://github.com/matanui159/ReplaySorcery) is an open-source, instant-replay solution for Linux.

[Luxtorpeda](https://github.com/luxtorpeda-dev/luxtorpeda) is a Steam Play compatibility tool to run games using native Linux engines.

[SteamGridDB](https://www.steamgriddb.com/projects) is a huge database where you can download and share custom video game assets and personalize your gaming library.

[SGDBoop](https://www.steamgriddb.com/boop) is a tool that automatically applies assets from SteamGridDB directly to your Steam library with a click of a button, removing the need to download and set them manually. [Available as a Flatpak](https://flathub.org/apps/details/com.steamgriddb.SGDBoop)

[Deck Verified](https://www.steamdeck.com/en/verified) is a program that reviews games in Steam's catalog verifying their compatibility with the Steam Deck. So when you visit your Library on Steam Deck, you’ll find a compatibility badge on each title, reflecting the kind of experience you can expect when playing each game on Steam Deck.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/142779573-254b3ce4-e0e8-401f-a343-bf5b3aa29b66.png">
<br />
</p>

Deck Verified Program Categories. Source: [Steam Deck](https://www.steamdeck.com/en/verified)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/142779574-d0410dc5-12cd-41ef-9cfa-03488c50b2ff.png">
<br />
</p>

Steam Library Compatibility Badges for Games. Source: [Steam Deck](https://www.steamdeck.com/en/verified)

## Installing Unreal Engine on Linux

[Back to the Top](#developers)

The easiest way to install Unreal Engine 5 on your Linux system is using the Epic Asset Manager that's availble on [FlatHub](https://flathub.org/) as a [Flatpak](https://flatpak.org). If you don't have Flatpak installed on your Linux system follow these [simple instructions to get started](https://flatpak.org/setup/).

[Epic Asset Manager](https://flathub.org/apps/details/io.github.achetagames.epic_asset_manager) is an unofficial client to install [Unreal Engine](https://www.unrealengine.com/), download and manage purchased assets, projects, plugins and games from the [Epic Games Store](https://www.unrealengine.com/marketplace?lang=en-US).

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/180886562-ef6aa63d-8117-4719-9af1-e25108042c2c.png">
<br />
</p>

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/180886564-3ebc8ecb-e2c0-4e53-9f45-f986bd24a89d.png">
<br />
</p>

### Quick links to Development Resources

[Back to the Top](#developers)

- [Sign-up for Epic Games Acount](https://www.epicgames.com/account/password)

- [Sign-up for Epic Games GitHub](https://github.com/EpicGames/Signup)

- [Unreal Engine 5 Linux Binary Download](https://www.unrealengine.com/en-US/linux)

- [Linux Development Requirements for Unreal Engine](https://docs.unrealengine.com/5.0/en-US/linux-development-requirements-for-unreal-engine/)

- [Unreal Engine Performance and Profiling](https://docs.unrealengine.com/5.0/en-US/TestingAndOptimization/PerformanceAndProfiling/)

- [Unreal Engine Blueprint API Reference](https://docs.unrealengine.com/5.0/en-US/BlueprintAPI/index.html)

- [Unreal Engine C++ API Reference](https://docs.unrealengine.com/5.0/en-US/API/index.html)

- [Unreal Engine Python API Reference](https://docs.unrealengine.com/5.0/en-US/PythonAPI/index.html)

## Game Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/97361059-45151700-185c-11eb-9d12-dae51c79eb8a.png">
  <br />
</p>

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/119279021-ea6b5000-bbdd-11eb-9f59-5251fc3ac751.png">
  <br />
</p>

## Game Engines

[Back to the Top](#developers)

**[Checkout the Unity Engine](https://unity.com/)**

<img src="https://user-images.githubusercontent.com/45159366/104788113-3432be00-5746-11eb-99b1-49360669f327.png">

**[Checkout the Unreal Engine 4](https://www.unrealengine.com/)**

<img src="https://user-images.githubusercontent.com/45159366/104788122-37c64500-5746-11eb-8f61-48a69b94582d.png">

**[Checkout the Godot Engine](https://godotengine.org/)**

[If you would like to Donate to the Godot Project](https://www.patreon.com/godotengine)

<img src="https://user-images.githubusercontent.com/45159366/104788134-3f85e980-5746-11eb-94c4-d97165ee888b.jpeg">

**[Checkout Blender](https://www.blender.org/)**

[If you would like to Donate to the Blender Project](https://fund.blender.org/)

<img src="https://user-images.githubusercontent.com/45159366/104788139-401e8000-5746-11eb-9647-058dee01a00e.png">

## Game Development Learning Resources

[Back to the Top](#developers)

[Unreal Online Learning](https://www.unrealengine.com/en-US/onlinelearning-courses) is a free learning platform that offers hands-on video courses and guided learning paths.

[Unreal Engine Authorized Training Program](https://www.unrealengine.com/en-US/training-partners)

[Unreal Engine for education](https://www.unrealengine.com/en-US/education/)

[Unreal Engine Training & Simulation](https://www.unrealengine.com/en-US/industry/training-simulation)

[Unity Certifications](https://unity.com/products/unity-certifications)

[Autodesk for Games](https://www.autodesk.com/campaigns/autodesk-for-games)

[Getting Started with DirectX 12 Ultimate](https://devblogs.microsoft.com/directx/directx-12-ultimate-getting-started-guide/)

[Getting Started with Vulkan](https://www.khronos.org/vulkan/)

[Getting Started with Apple Metal](https://developer.apple.com/metal/)

[Game Design Online Courses from Udemy](https://www.udemy.com/courses/Design/Game-Design/)

[Game Design Online Courses from Skillshare](https://www.skillshare.com/browse/game-design)

[Learn Game Design with Online Courses and Classes from edX](https://www.edx.org/learn/game-design)

[Game Design Courses from Coursera](https://www.coursera.org/courses?query=game%20design)

[Game Design and Development Specialization Course from Coursera](https://www.coursera.org/specializations/game-development)

## Game Development Tools

[Back to the Top](#developers)

[Unreal Engine](https://www.unrealengine.com) is a game engine developed by Epic Games with the world's most open and advanced real-time 3D creation tool. Continuously evolving to serve not only its original purpose as a state-of-the-art game engine, today it gives creators across industries the freedom and control to deliver cutting-edge content, interactive experiences, and immersive virtual worlds.

[Unity](https://unity.com) is a cross-platform game development platform. Use Unity to build high-quality 3D and 2D games, deploy them across mobile, desktop, VR/AR, consoles or the Web, and connect with loyal and enthusiastic players and customers.

[Unigine](https://unigine.com) is a cross-platform game engine designed for development teams (C++/C# programmers, 3D artists) working on interactive 3D apps.

[Blender](https://www.blender.org) is the free and open source 3D creation suite. It supports the entirety of the 3D pipeline—modeling, rigging, animation, simulation, rendering, compositing and motion tracking, video editing and 2D animation pipeline.

[Godot](https://godotengine.org) is a feature-packed, cross-platform game engine to create 2D and 3D games from a unified interface. It provides a comprehensive set of common tools, so that users can focus on making games without having to reinvent the wheel. Games can be exported in one click to a number of platforms, including the major desktop platforms (Linux, Mac OSX, Windows) as well as mobile (Android, iOS) and web-based (HTML5) platforms.

[Panda3D](https://www.panda3d.org/) is a game engine, a framework for 3D rendering and game development for Python and C++ programs, developed by Disney and CMU. Panda3D is open-source and free for any purpose, including commercial ventures.

[Source 2](https://developer.valvesoftware.com/wiki/Source_2) is a 3D video game engine in development by Valve as a successor to Source. It is used in Dota 2, Artifact, Dota Underlords, parts of The Lab, SteamVR Home, and Half-Life: Alyx.

[Havok](https://www.havok.com/) is a middleware software suite that provides a realistic physics engine component and related functions to video games. It is supported  and optimized across all major platforms, including Nintendo Switch, PlayStation®, Stadia, and Xbox. Along with integrations for Unity and Unreal Engine and are used in countless proprietary game engines.

[AutoDesk 3ds Max](https://www.autodesk.com/products/3ds-max/overview) is a professional software program for 3D modeling, animation, rendering, and visualization. 3ds Max allows you to create stunning game environments, design visualizations, and virtual reality experiences.

[Houdini](https://www.sidefx.com/) is a 3D procedural software for modeling, rigging, animation, VFX, look development, lighting and rendering in film, TV, advertising and video game pipelines.

[A-Frame](https://aframe.io) is a web framework for building virtual reality experiences in WebVR with HTML and Entity-Component. A-Frame works on Vive, Rift, desktop, mobile platforms.

[AppGameKit](https://www.appgamekit.com) is a powerful game development engine, ideal for Hobbyist and Indie developers. Where you can start coding in the easy to learn AppGameKit BASIC or use the libraries in C++ & XCode.

[Open Graphics Library(OpenGL)](https://www.opengl.org/) is an API used acrossed mulitple  programming languages and platforms for hardware-accelerated rendering of 2D/3D vector graphics currently developed by the [Khronos Group](https://www.khronos.org/).

[Open Computing Language (OpenCL)](https://www.khronos.org/opencl/) is an open standard for [parallel programming](https://www.coursera.org/lecture/parprog1/introduction-to-parallel-computing-zNrIS) of heterogeneous platforms consisting of CPUs, GPUs, and other hardware accelerators found in supercomputers, cloud servers, personal computers, mobile devices and embedded platforms.

[OpenGL Shading Language(GLSL)](https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)) is a High Level Shading Language based on the C-style language, so it covers most of the features a user would expect with such a language.  Such as control structures (for-loops, if-else statements, etc) exist in GLSL, including the switch statement.

[High Level Shading Language(HLSL)](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-hlsl) is the High Level Shading Language for DirectX. Using HLSL, the user can create C-like programmable shaders for the Direct3D pipeline. HLSL was first created with DirectX 9 to set up the programmable 3D pipeline.

[DirectX 12 Ultimate](https://github.com/Microsoft/DirectX-Graphics-Samples) is an API(for high performance 2D & 3D graphics) from Microsoft. DirectX 12 Ultimate brings support for ray tracing, mesh shaders, variable rate shading, and sampler feedback. Available in Windows 2004 version(May 2020 Update).

[Vulkan](https://www.khronos.org/vulkan/) is a modern cross-platform graphics and compute API that provides high-efficiency, cross-platform access to modern GPUs used in a wide variety of devices from PCs and consoles to mobile phones and embedded platforms. Vulkan is currently in development by the Khronos Group consortium.

[Mesa 3D Graphics Library](https://docs.mesa3d.org/index.html) is a project began as an open-source implementation of the OpenGL specification. A system for rendering interactive 3D graphics. Mesa ties into several other open-source projects: the [Direct Rendering Infrastructure](https://dri.freedesktop.org/), [X.org](https://x.org/), and [Wayland](https://wayland.freedesktop.org/) to provide OpenGL support on Linux, FreeBSD, and other operating systems.

[OpenGL ES](https://www.khronos.org/opengles/) is the mobile subset of OpenGL. It's supported on all major mobile platforms, and is also the base for WebGL.

[OpenCL](https://www.khronos.org/opencl/) is a framework for writing programs that execute across heterogeneous platforms consisting of CPUs, GPUs, DSPs, FPGAs and other processors or hardware accelerators.

[EGL](https://www.khronos.org/egl/) is an interface between Khronos rendering APIs such as OpenGL or OpenVG and the underlying native platform window system.

[VDPAU](https://www.freedesktop.org/wiki/Software/VDPAU/) is the Video Decode and Presentation API for UNIX. It provides an interface to video decode acceleration and presentation hardware present in modern GPUs.

[VA API](https://freedesktop.org/wiki/Software/vaapi/) is an open-source library and API specification, which provides access to graphics hardware acceleration capabilities for video processing.

[XvMC](https://en.wikipedia.org/wiki/X-Video_Motion_Compensation) is an extension of the X video extension (Xv) for the X Window System. The XvMC API allows video programs to offload portions of the video decoding process to the GPU hardware.

[AMD Radeon ProRender](https://www.amd.com/en/technologies/radeon-prorender) is a powerful physically-based rendering engine that enables creative professionals to produce stunningly photorealistic images on virtually any GPU, any CPU, and any OS in over a dozen leading digital content creation and CAD applications.

[NVIDIA Omniverse](https://developer.nvidia.com/nvidia-omniverse-platform) is a powerful, multi-GPU, real-time simulation and collaboration platform for 3D production pipelines based on Pixar's Universal Scene Description and NVIDIA RTX.

[LibGDX](https://github.com/libgdx/libgdx) is a cross-platform Java game development framework based on OpenGL (ES) that works on Windows, Linux, Mac OS X, Android, your WebGL enabled browser and iOS.

[cocos2d-x](https://github.com/cocos2d/cocos2d-x) is a multi-platform framework for building 2d games, interactive books, demos and other graphical applications. It is based on cocos2d-iphone, but instead of using Objective-C, it uses C++. It works on iOS, Android, macOS, Windows and Linux.

[MonoGame](https://github.com/MonoGame/MonoGame) is a framework for creating powerful cross-platform games. The spiritual successor to XNA with thousands of titles shipped across desktop, mobile, and console platforms. MonoGame is a fully managed .NET open source game framework without any black boxes.

[Three.js](https://threejs.org) is a cross-browser JavaScript library and application programming interface used to create and display animated 3D computer graphics in a web browser using WebGL.

[Superpowers](http://superpowers-html5.com/) is a downloadable HTML5 app for real-time collaborative projects . You can use it solo like a regular offline game maker, or setup a password and let friends join in on your project through their Web browser.

[URHO3D](https://urho3d.github.io/) is a free lightweight, cross-platform 2D and 3D game engine implemented in C++ and released under the MIT license. Greatly inspired by OGRE and Horde3D.

[Vivox](https://www.vivox.com/) is a voice & text chat platform that's trusted by the world's biggest gaming brands and titles such as Fortnite, PUBG, League of Legends, and Rainbow Six Siege.

[HGIG](https://www.hgig.org/) is a volunteer group of companies from the game and TV display industries that meet to specify and make available for the public guidelines to improve consumer gaming experiences in HDR.

[GameBlocks](https://www.gameblocks.com/) is a Server Side Anti-Cheat & Middleware software.

## Vulkan Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/129622224-8c4cca51-9200-4d70-9d16-2610d704713a.png">
  <br />
</p>

## Vulkan Learning Resources

[Back to the Top](#developers)

[Vulkan®](https://www.khronos.org/vulkan/) is a modern cross-platform graphics and compute API that provides high-efficiency, cross-platform access to modern GPUs used in a wide variety of devices from PCs and consoles to mobile phones and embedded platforms. Vulkan is currently in development by the Khronos consortium.

[Khronos Group GitHub](https://github.com/KhronosGroup)

[Vulkan Documentation](https://github.com/KhronosGroup/Vulkan-Docs)

[HLSL to SPIR-V Feature Mapping Manual](https://github.com/microsoft/DirectXShaderCompiler/blob/master/docs/SPIR-V.rst)

[Vulkan GLSL Ray Tracing Emulator Tutorial](https://www.gsn-lib.org/docs/nodes/raytracing.php)

[Getting Started with Vulkan](https://vulkan-tutorial.com/)

[Vulkan Samples](https://github.com/KhronosGroup/Vulkan-Samples)

[Khronos Community Forums](https://community.khronos.org/)

## Vulkan Tools, Libraries, and Frameworks

[Back to the Top](#developers)

[Vulkan SDK](https://vulkan.lunarg.com) is a set of tools that enables Vulkan developers to develop Vulkan applications.

[SPIR-V](https://www.khronos.org/spir/) is a set of tools that enables high-level language front-ends to emit programs in a standardized intermediate form to be ingested by Vulkan, OpenGL or OpenCL drivers. It eliminates the need for high-level language front-end compilers in device drivers, significantly reducing driver complexity, enables a broad range of language and framework front-ends to run on diverse hardware architectures and encourages a vibrant ecosystem of open source analysis, porting, debug and optimization tools.

[SPIRV-Reflect](https://github.com/KhronosGroup/SPIRV-Reflect) is a lightweight library that provides a C/C++ reflection API for SPIR-V shader bytecode in Vulkan applications.

[Vulkan® Tools](https://github.com/KhronosGroup/Vulkan-Tools) is a project that provides Khronos Group's official Vulkan Tools and Utilities for Windows, Linux, Android, and macOS.

[Vulkan-Hpp](https://github.com/KhronosGroup/Vulkan-Hpp) is a API that provides a header only C++ bindings for the Vulkan C API to improve the developers Vulkan experience without introducing CPU runtime cost. It adds features like type safety for enums and bitfields, STL container support, exceptions and simple enumerations.

[Vulkan® Memory Allocator (VMA)](https://gpuopen.com/vulkan-memory-allocator/) is a  library that provides a simple and easy to integrate API to help you allocate memory for Vulkan® buffer and image storage.

[AMD Open Source Driver for Vulkan®](https://gpuopen.com/amd-open-source-driver-for-vulkan/) is an open-source Vulkan driver for AMD Radeon™ graphics adapters on Linux®.

[NVIDIA® Nsight™ Visual Studio Edition](https://developer.nvidia.com/nsight-visual-studio-edition) is an application development environment for heterogeneous platforms which brings GPU computing into Microsoft Visual Studio. NVIDIA Nsight™ VSE allows you to build and debug integrated GPU kernels and native CPU code as well as inspect the state of the GPU and memory.

[Radeon™ GPU Profiler](https://gpuopen.com/rgp/) is a performance tool that can be used by developers to optimize DirectX®12, Vulkan® and OpenCL™ applications for AMD RDNA™ and GCN hardware.

[Radeon™ GPU Analyzer](https://gpuopen.com/rga/) is a compiler and code analysis tool for Vulkan®, DirectX®, OpenGL® and OpenCL™.

[Radeon™ Memory Visualizer (RMV)](https://gpuopen.com/rmv/) is a tool provided by AMD for use by game engine developers. It allows engineers to examine, diagnose, and understand the GPU memory management within their projects.

[DXVK](https://github.com/doitsujin/dxvk) is a Vulkan-based translation layer for Direct3D 9/10/11 which allows running 3D applications on Linux using Wine.

[MoltenVK](https://moltengl.com/moltenvk) is an implementation of Vulkan running on iOS and macOS using Apple's [Metal](https://developer.apple.com/metal/) graphics framework.

[RenderDoc](https://renderdoc.org) is a stand-alone graphics debugger that allows quick and easy single-frame capture and detailed introspection of any application using Vulkan, D3D11, OpenGL & OpenGL ES or D3D12 across Windows, Linux, Android, Stadia, or Nintendo Switch™.

[PerfDoc](https://github.com/ARM-software/perfdoc) is a cross-platform Vulkan layer which checks Vulkan applications for [best practices on Arm Mali](https://developer.arm.com/graphics/developer-guides/mali-gpu-best-practices) devices.

[GLFW](https://www.glfw.org/) is an Open Source, multi-platform library for OpenGL, OpenGL ES and Vulkan application development. It provides a simple, platform-independent API for creating windows, contexts and surfaces, reading input, handling events, etc. GLFW natively supports Windows, macOS and Linux and other Unix-like systems. On Linux both X11 and Wayland are supported.

[VulkanSharp](https://github.com/mono/VulkanSharp) is a project provides a .NET binding for the Vulkan API.

[Vortice.Vulkan](https://github.com/amerkoleci/Vortice.Vulkan) is a .NET Standard 2.0 and .NET5 low-level bindings for Vulkan API.

[VKD3D-Proton](https://github.com/HansKristian-Work/vkd3d-proton) is a fork of VKD3D, which aims to implement the full Direct3D 12 API on top of Vulkan.

[ImGui](https://github.com/ocornut/imgui) is a bloat-free graphical user interface library for C++. It outputs optimized vertex buffers that you can render anytime in your 3D-pipeline enabled application. It is fast, portable, renderer agnostic and self-contained (no external dependencies).

[Ash](https://github.com/MaikKlein/ash) is a very lightweight wrapper around Vulkan.

[gfx-rs](https://github.com/gfx-rs/gfx) is a low-level, cross-platform graphics and compute abstraction library in Rust.

[Vulkan.jl](https://github.com/JuliaGPU/Vulkan.jl) is a lightweight wrapper around the Vulkan graphics and compute library. It exposes abstractions over the underlying C interface, primarily geared towards developers looking for a more natural way to work with Vulkan with minimal overhead.

## DirectX Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779599-6a46ab05-c64a-48fe-a775-3e4f46e41f63.png">
  <br />
</p>

## DirectX Learning Resources

[Back to the Top](#developers)

[Microsoft DirectX®](https://support.microsoft.com/en-us/topic/how-to-install-the-latest-version-of-directx-d1f5ffa5-dae2-246c-91b1-ee1e973ed8c2) is a low-level API that handles tasks related to multimedia for game programming and video on Microsoft platforms(Windows & Xbox).

[Getting Started with DirectX 12 Ultimate](https://devblogs.microsoft.com/directx/directx-12-ultimate-getting-started-guide/)

[Getting Started with the DirectX 12 Agility SDK](https://devblogs.microsoft.com/directx/gettingstarted-dx12agility/)

[DirectX 12 and Graphics Education | YouTube](https://www.youtube.com/channel/UCiaX2B8XiXR70jaN7NK-FpA)

[DirectX— Feature Level 12_2](https://devblogs.microsoft.com/directx/new-in-directx-feature-level-12_2/)

[DirectX 12 Technology | NVIDIA](https://www.nvidia.com/en-us/geforce/technologies/dx12/)

[AMD DirectX® 12 (DX12) Technology | AMD](https://www.amd.com/en/technologies/directx12)

[Top Microsoft DirectX Courses Online | Udemy](https://www.udemy.com/topic/microsoft-directx/)

[DirectX - Learn Microsoft DirectX from Scratch Course | Udemy](https://www.udemy.com/course/directx-learn-microsoft-directx-from-scratch/)

[DirectX 11 Programming Course | Udemy](https://www.udemy.com/course/directx11/)

## DirectX Tools, Libraries, and Frameworks

[Back to the Top](#developers)

[Visual Studio](https://visualstudio.microsoft.com/) is an integrated development environment (IDE) from Microsoft; which is a feature-rich application that can be used for many aspects of software development. Visual Studio makes it easy to edit, debug, build, and publish your app. By using Microsoft software development platforms such as Windows API, Windows Forms, Windows Presentation Foundation, and Windows Store.

[Visual Studio Code](https://code.visualstudio.com/) is a code editor redefined and optimized for building and debugging modern web and cloud applications.

[DirectX-Graphics-Samples](https://github.com/Microsoft/DirectX-Graphics-Samples) is a project that contains the DirectX 12 Graphics samples that demonstrate how to build graphics intensive applications for Windows 10.

[PIX on Windows](https://devblogs.microsoft.com/pix/documentation/) is a performance tuning and debugging tool for DirectX 12 games on Windows.

[DirectStorage API](https://devblogs.microsoft.com/directx/directstorage-is-coming-to-pc/) is an API in the DirectX family originally designed for the [Velocity Architecture](https://news.xbox.com/en-us/2020/07/14/a-closer-look-at-xbox-velocity-architecture/) to Windows. The DirectX API is architected in a way that takes all this into account and maximizes performance throughout the entire pipeline from NVMe drive all the way to the GPU. It does this in several ways: by reducing per-request NVMe overhead, enabling batched many-at-a-time parallel IO requests which can be efficiently fed to the GPU, and giving games finer grain control over when they get notified of IO request completion instead of having to react to every tiny IO completion. The DirectStorage API will be available on [Windows 11](https://www.microsoft.com/en-us/windows/windows-11) PCs with NVMe SSDs, but will also be support in [Windows 10](https://www.microsoft.com/software-download/windows10) version 1909 and newer.

[NVIDIA® Nsight™ Visual Studio Edition](https://developer.nvidia.com/nsight-visual-studio-edition) is an application development environment for heterogeneous platforms which brings GPU computing into Microsoft Visual Studio. NVIDIA Nsight™ VSE allows you to build and debug integrated GPU kernels and native CPU code as well as inspect the state of the GPU and memory.

[NVRHI (NVIDIA Rendering Hardware Interface)](https://github.com/NVIDIAGameWorks/nvrhi) is a library that implements a common abstraction layer over multiple graphics APIs (GAPIs): Direct3D 11, Direct3D 12, and Vulkan 1.2. It works on Windows (x64 only) and Linux (x64 and ARM64).

[RTXMU - RTX Memory Utility SDK](https://github.com/NVIDIAGameWorks/RTXMU) is an SDK tool that batchs up all of the acceleration structure build inputs and pass them to RTXMU which in turn will perform all the suballocation memory requests and build details including compaction. Then post build info is abstracted away by the SDK in order to do compaction under the hood. RTXMU returns acceleration structure handle ids that are used to reference the underlying memory buffers. These handle ids are passed into RTXMU to create compaction copy workloads, deallocate unused build resources or remove all memory associated with an acceleration structure.

[Radeon™ GPU Profiler](https://gpuopen.com/rgp/) is a performance tool that can be used by developers to optimize DirectX®12, Vulkan® and OpenCL™ applications for AMD RDNA™ and GCN hardware.

[Radeon™ GPU Analyzer](https://gpuopen.com/rga/) is a compiler and code analysis tool for Vulkan®, DirectX®, OpenGL® and OpenCL™.

[Radeon™ Memory Visualizer (RMV)](https://gpuopen.com/rmv/) is a tool provided by AMD for use by game engine developers. It allows engineers to examine, diagnose, and understand the GPU memory management within their projects.

[FNA](https://fna-xna.github.io/) is an XNA4 reimplementation that focuses solely on developing a fully accurate XNA4 runtime for the desktop.

[FAudio](https://fna-xna.github.io/) is an XAudio reimplementation that focuses solely on developing fully accurate DirectX Audio runtime libraries for the FNA project, including [XAudio2](https://docs.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-introduction), [X3DAudio](https://docs.microsoft.com/en-us/windows/win32/xaudio2/x3daudio-overview), [XAPO](https://docs.microsoft.com/en-us/windows/win32/xaudio2/xapo-overview), and [XACT3](https://en.wikipedia.org/wiki/Cross-platform_Audio_Creation_Tool).

[Simple DirectMedia Layer](https://www.libsdl.org/) is a cross-platform development library designed to provide low level access to audio, keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D. It is used by video playback software, emulators, and popular games including Valve's award winning catalog.

[DXVK](https://github.com/doitsujin/dxvk) is a Vulkan-based translation layer for Direct3D 9/10/11 which allows running 3D applications on Linux using Wine.

[VKD3D-Proton](https://github.com/HansKristian-Work/vkd3d-proton) is a fork of VKD3D, which aims to implement the full Direct3D 12 API on top of Vulkan.

[RenderDoc](https://renderdoc.org) is a stand-alone graphics debugger that allows quick and easy single-frame capture and detailed introspection of any application using Vulkan, D3D11, OpenGL & OpenGL ES or D3D12 across Windows, Linux, Android, Stadia, or Nintendo Switch™.

## OpenGL Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/131386211-f507b5d4-a3c9-4c21-aadd-2aa5bde94d1e.png">
  <br />
</p>

## OpenGL Learning Resources

[Back to the Top](#developers)

[Open Graphics Library(OpenGL)™](https://www.opengl.org/) is an API used across multiple  programming languages and platforms for hardware-accelerated rendering of 2D/3D vector graphics currently developed by the [Khronos Group](https://www.khronos.org/).

[OpenGL ES™](https://www.khronos.org/opengles/) is the mobile subset of OpenGL for Embedded Systems(ES). It's supported on all major mobile platforms, and is also the base for WebGL.

[WebGL™](https://www.khronos.org/webgl/) is a cross-platform, royalty-free web standard for a low-level 3D graphics API based on OpenGL ES, exposed to JavaScript via the HTML5 Canvas element.

[Khronos Group | GitHub](https://github.com/KhronosGroup/)

[Khronos Technology Courses and Training](https://www.khronos.org/developers/training/)

[Top OpenGL Courses Online | Coursera](https://www.coursera.org/courses?query=opengl&amp;page=1)

[Top OpenGL Courses Online | Udemy](https://www.udemy.com/topic/opengl/)

[OpenGL Online Training Courses | LinkedIn Learning](https://www.linkedin.com/learning/topics/opengl)

[Getting Started with OpenGL](https://www.khronos.org/opengl/wiki/Getting_Started)

[OpenGL Reference Cards](https://www.khronos.org/developers/reference-cards/)

[Getting Started with OpenGL ES](https://www.khronos.org/opengles/)

[OpenGL ES Reference Cards](https://www.khronos.org/developers/reference-cards/)

[Getting Started with WebGL](https://www.khronos.org/webgl/)

[WebGL 2.0 Specification](https://www.khronos.org/registry/webgl/specs/latest/2.0/)

[WebGL Public Wiki](https://www.khronos.org/webgl/wiki)

[WebGL Reference Cards](https://www.khronos.org/developers/reference-cards/)

## OpenGL Tools, Libraries, and Frameworks

[Back to the Top](#developers)

[BuGLe](https://www.opengl.org/sdk/tools/BuGLe/) is a debugger for Linux and other UNIX-like OSes. BuGLe combines a graphical OpenGL debugger with a selection of filters on the OpenGL command stream. The debugger allows viewing of state, textures, framebuffers and shaders, while the filters allow for logging, error checking, video capture and more.

[gDEBugger](https://www.opengl.org/sdk/tools/gDEBugger/) is a full-featured and free debugger and profiler representing the state-of-the-art in OpenGL and OpenGL ES debugging and profiling on  Windows and Linux.

[KTX](http://www.khronos.org/opengles/sdk/tools/KTX/) is a lightweight file format for delivering textures to OpenGL family APIs.

[RenderDoc](https://renderdoc.org) is a stand-alone graphics debugger that allows quick and easy single-frame capture and detailed introspection of any application using Vulkan, D3D11, OpenGL & OpenGL ES or D3D12 across Windows, Linux, Android, Stadia, or Nintendo Switch™.

[NVIDIA® Nsight™ Visual Studio Edition](https://developer.nvidia.com/nsight-visual-studio-edition) is an application development environment for heterogeneous platforms which brings GPU computing into Microsoft Visual Studio. NVIDIA Nsight™ VSE allows you to build and debug integrated GPU kernels and native CPU code as well as inspect the state of the GPU and memory.

[Radeon™ GPU Profiler](https://gpuopen.com/rgp/) is a performance tool that can be used by developers to optimize DirectX®12, Vulkan® and OpenCL™ applications for AMD RDNA™ and GCN hardware.

[Radeon™ GPU Analyzer](https://gpuopen.com/rga/) is a compiler and code analysis tool for Vulkan®, DirectX®, OpenGL® and OpenCL™.

[AMD Radeon ProRender](https://www.amd.com/en/technologies/radeon-prorender) is a powerful physically-based rendering engine that enables creative professionals to produce stunningly photorealistic images on virtually any GPU, any CPU, and any OS in over a dozen leading digital content creation and CAD applications.

[NVIDIA Omniverse](https://developer.nvidia.com/nvidia-omniverse-platform) is a powerful, multi-GPU, real-time simulation and collaboration platform for 3D production pipelines based on Pixar's Universal Scene Description and NVIDIA RTX.

[MoltenGL](https://moltengl.com) is an implementation of the OpenGL ES 2.0 API that runs on Apple's [Metal](https://developer.apple.com/metal/) graphics framework.

[EGL](https://www.khronos.org/egl/) is an interface between Khronos rendering APIs such as OpenGL or OpenVG and the underlying native platform window system.

[Equalizer](https://www.opengl.org/sdk/libs/Equalizer/) is an open source programming interface and resource management system for scalable OpenGL applications. An Equalizer application can be deployed on any visualization system, from a single-pipe workstation to large scale graphics clusters.

[GLee](https://www.opengl.org/sdk/libs/GLee/) is a free cross-platform extension loading library that takes the burden off your application. GLee makes it easy to check for OpenGL extension and core version availability, automatically setting up the entry points with no effort on your part.

[GLEW](https://www.opengl.org/sdk/libs/GLEW/) is an open-source cross-platform extension loading library with thread-safe support for multiple rendering contexts and automatic code generation capability. GLEW provides easy-to-use and efficient methods for checking OpenGL extensions and core functionality.

[GLUS](https://www.opengl.org/sdk/libs/GLUS) is an open-source C library, which provides a hardware and operating system abstraction plus many functions usually needed for graphics programming using OpenGL, OpenGL ES or OpenVG.

[OpenGL Mathematics (GLM)](http://glm.g-truc.net/) is a C++ mathematics library for 3D software based on the OpenGL Shading Language (GLSL) specification.

[libktx](http://www.khronos.org/opengles/sdk/tools/KTX/index.php#libktx) is a library of functions(part of the [KTX tool set](http://www.khronos.org/opengles/sdk/tools/KTX)) for writing [KTX format files](http://www.khronos.org/opengles/sdk/tools/KTX/file_format_spec/) and instantiating GL textures from them.

[OpenSceneGraph](https://www.opengl.org/sdk/libs/OpenSceneGraph/) is a high-level 3D graphics toolkit exposing OpenGL's capabilities while providing many capabilities of its own. OpenSceneGraph boasts a large user community and has been employed for visual simulation, games, virtual reality, scientific visualization, and modeling.

[Mesa 3D Graphics Library](https://docs.mesa3d.org/index.html) is a project that began as an open-source implementation of the OpenGL specification. A system for rendering interactive 3D graphics. Mesa ties into several other open-source projects: the [Direct Rendering Infrastructure](https://dri.freedesktop.org/), [X.org](https://x.org/), and [Wayland](https://wayland.freedesktop.org/) to provide OpenGL support on Linux, FreeBSD, and other operating systems.

## Wayland Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/104235197-79cf4e00-5409-11eb-97a6-a12f7bd8ad2a.png">
  <br />
</p>

## Wayland Learning Resources

[Back to the Top](#developers)

[Wayland](https://wayland.freedesktop.org) is a protocol for a compositor to talk to its clients as well as a C library implementation of that protocol. The compositor can be a standalone display server running on Linux kernel modesetting and evdev input devices, an [X application](https://www.x.org/wiki/XServer/), or a Wayland client itself.

[Wayland Architecture](https://wayland.freedesktop.org/architecture.html)

[Wayland Documentation](https://wayland.freedesktop.org/docs/html/)

[Sotfware Toolkits that have Wayland support right now](https://wayland.freedesktop.org/toolkits.html)

[Contribution instructions for Wayland](https://gitlab.freedesktop.org/wayland/wayland/blob/master/CONTRIBUTING.md)

[Contribution instructions for Weston](https://gitlab.freedesktop.org/wayland/weston/blob/master/CONTRIBUTING.md)

[Reporting Wayland bugs](https://gitlab.freedesktop.org/wayland/wayland/issues)

[Reporting Weston bugs](https://gitlab.freedesktop.org/wayland/weston/issues)

[WSLG: X11 and Wayland Applications in Windows Subsystem for Linux(WSL2)](https://linuxplumbersconf.org/event/9/contributions/611/attachments/702/1298/XDC2020_-_X11_and_Wayland_applications_in_WSL.pdf)

[Qt Wayland Compositor](https://doc.qt.io/qt-5/qtwaylandcompositor-index.html)

[Qt Wayland Compositor Examples](https://doc.qt.io/qt-5/qtwaylandcompositor-examples.html)

[Wayland on ArchWiki](https://wiki.archlinux.org/index.php/Wayland)

[Sway on ArchWiki](https://wiki.archlinux.org/index.php/Sway)

[Wayland on Ubuntu Wiki](https://wiki.ubuntu.com/Wayland)

[Wayland on Debian Wiki](https://wiki.debian.org/Wayland)

[The Wayland Display Server on Fedora Docs](https://docs.fedoraproject.org/en-US/fedora/rawhide/system-administrators-guide/Wayland/)

[Wayland features on Fedora Project Wiki](https://fedoraproject.org/wiki/Wayland_features)

[Wayland on GNOME Wiki](https://wiki.gnome.org/Initiatives/Wayland)

[KWin/Wayland on KDE Community Wiki](https://community.kde.org/index.php?title=KWin/Wayland)

[Wayland Desktop Landscape on Gentoo Wiki](https://wiki.gentoo.org/wiki/Wayland_Desktop_Landscape)

[Wayland in Void Linux Handbook](https://docs.voidlinux.org/config/graphical-session/wayland.html)

[Wayland on Enlightenment DE](https://www.enlightenment.org/about-wayland)

## Wayland Tools

[Back to the Top](#developers)

[Weston](https://gitlab.freedesktop.org/wayland/weston) is a lightweight and functional Wayland compositor.

[XWayland](https://wayland.freedesktop.org/xserver.html) is an X Server running as a Wayland client(for backwards compatibility), allowing the [Xorg server](https://www.x.org/wiki/XServer/) can be modified to use wayland input devices for input and forward either the root window or individual top-level windows as wayland surfaces.

[KWin](https://community.kde.org/KWin/Wayland) is the window manager for the KDE Plasma Desktop. It gives you complete control over your windows, making sure they're not in the way but aid you in your task. It paints the window decoration, the bar on top of every window with (configurable) buttons like close, maximize and minimize.

[Qt](https://www.qt.io/) is the faster, smarter way to create innovative devices, modern UIs & applications for multiple screens. It is one of the most popular toolkits for the Wayland and X11 windowing.

[GTK](https://www.gtk.org/) is a free and open source cross-platform widget toolkit for creating graphical user interfaces developed by [GNOME Project](https://www.gnome.org/). It is one of the most popular toolkits for the Wayland and X11 windowing.

[NVIDIA Wayland EGL External Platform library](https://github.com/NVIDIA/egl-wayland) is a work-in-progress implementation of a EGL External Platform library to add client-side Wayland support to EGL on top of EGLDevice and EGLStream families of extensions.

[NVIDIA EGL External Platform Interface](https://github.com/NVIDIA/eglexternalplatform) is a work-in-progress specification of the EGL External Platform interface for writing EGL platforms and their interactions with modern window systems on top of existing low-level EGL platform implementations. This keeps window system implementation specifics out of EGL drivers by using application-facing EGL functions.

[Sway](https://swaywm.org/) is an [i3](https://i3wm.org/)-compatible Wayland compositor.

[wlroots](https://github.com/swaywm/wlroots) is a modular Wayland compositor library.

[WayfireWM](https://github.com/WayfireWM/wayfire) is a 3D Wayland compositor, inspired by [Compiz](https://launchpad.net/compiz) and based on [wlroots](https://github.com/swaywm/wlroots).

[SDDM](https://github.com/sddm/sddm) is a modern display manager for X11 and Wayland aiming to be fast, simple and beautiful. It uses modern technologies like QtQuick, which in turn gives the designer the ability to create smooth, animated user interfaces.

[x11docker](https://github.com/mviereck/x11docker) is an application that you allows to run graphical desktop applications (and entire desktops) in Docker Linux containers.

[Mako](https://github.com/emersion/mako) is a lightweight notification daemon for Wayland. It also works on [Sway](https://swaywm.org/).

[Wayland-rs](https://github.com/Smithay/wayland-rs) is a Rust implementation of the wayland protocol (client and server).

[Wine-wayland](https://github.com/varmd/wine-wayland) is an application that allows you to running DX9/DX11 and Vulkan games using pure Wayland and Wine/DXVK.

## Audio Development

[Back to the Top](#developers)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142936394-b546784e-231a-4391-9dd8-c686e5a7eee9.png">
  <br />
</p>

## Audio Learning Resources

[Back to the Top](#developers)

[How Audio is implemented with PipeWire](https://docs.pipewire.org/page_audio.html)

[PipeWire Design](https://docs.pipewire.org/page_pipewire.html)

[PipeWire Library](https://docs.pipewire.org/page_library.html)

[DMA-BUF sharing](https://docs.pipewire.org/page_dma_buf.html)

[PipeWire Daemon](https://docs.pipewire.org/page_daemon.html)

[PipeWire Tools](https://docs.pipewire.org/page_tools.html)

[PipeWire Session Manager](https://docs.pipewire.org/page_session_manager.html)

[PipeWire Modules](https://docs.pipewire.org/page_pipewire_modules.html)

[PipeWire API](https://docs.pipewire.org/page_api.html)

- [Client Implementation](https://docs.pipewire.org/page_client_impl.html)
- [Proxy](https://docs.pipewire.org/page_proxy.html)
- [Streams](https://docs.pipewire.org/page_streams.html)
- [Thread Loop](https://docs.pipewire.org/page_thread_loop.html)

[WirePlumber Documentation](https://pipewire.pages.freedesktop.org/wireplumber/)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779615-d631251b-a2d6-48b4-8194-7985604a8563.png">
  <br />
</p>

How WirePlumber, the PipeWire session manager works. Source: [Collabora](https://www.collabora.com/news-and-blog/blog/2020/05/07/wireplumber-the-pipewire-session-manager/)

## Audio Tools

[Back to the Top](#developers)

[PipeWire](https://pipewire.org) is a server and user space API to deal with multimedia pipelines. It provides a low-latency, graph based processing engine on top of audio and video devices that can be used to support the use cases currently handled by both pulseaudio and JACK. PipeWire was designed with a powerful security model that makes interacting with audio and video devices from containerized applications easy. Nodes in the graph can be implemented as separate processes, communicating with sockets and exchanging multimedia content using fd passing.

[WirePlumber](https://pipewire.pages.freedesktop.org/wireplumber/) is a modular session / policy manager for [PipeWire](https://pipewire.org/) and a GObject-based high-level library that wraps PipeWire’s API, providing convenience for writing the daemon’s modules as well as external tools for managing PipeWire. The WirePlumber daemon implements the session & policy management service. It follows a modular design, having plugins that implement the actual management functionality.

[Core API](https://docs.pipewire.org/group__api__pw__core.html) is used by all clients that need to communicate with the [PipeWire Daemon](https://docs.pipewire.org/page_daemon.html) and provides the necessary structs to interface with the daemon.

[Implementation API](https://docs.pipewire.org/group__api__pw__impl.html) is primarily used by the [PipeWire Daemon](https://docs.pipewire.org/page_daemon.html) itself but also by the [PipeWire Session Manager](https://docs.pipewire.org/page_session_manager.html) and modules/extensions that need to build objects in the graph.

[SPA (Simple Plugin API)](https://docs.pipewire.org/group__api__spa.html) is an extensible API to implement all kinds of plugins.

[GStreamer](https://gstreamer.freedesktop.org/) is a library for constructing graphs of media-handling components. The applications it supports range from simple Ogg/Vorbis playback, audio/video streaming to complex audio (mixing) and video (non-linear editing) processing. Applications can take advantage of advances in codec and filter technology transparently.

[Media Source Extensions (MSE)](https://www.w3.org/TR/media-source/) is a [W3C specification](https://github.com/w3c/media-source) that allows JavaScript to send byte streams to media codecs within Web browsers that support HTML5 video and audio. Also, this allows the implementation of client-side prefetching and buffering code for streaming media entirely in JavaScript.

[WebRTC](https://webrtc.org/) is an open-source project that adds real-time communication capabilities to your application that works on top of an open standard. It supports video, voice, and generic data to be sent between peers, allowing developers to build powerful voice- and video-communication solutions.
