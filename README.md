<h1 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779553-82147e51-7e6d-47bd-9db6-fe2f5ad95355.png">
  <br />
  Steam Deck Guide
</h1>

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/142779557-400f19c8-6084-41ee-9449-fb08a15d6c45.png">
<br />
</p>


#### A guide covering Steam Deck including the applications and tools that will make you a better and more efficient with your Steam Deck device.

**Note: You can easily convert this markdown file to a PDF in [VSCode](https://code.visualstudio.com/) using this handy extension [Markdown PDF](https://marketplace.visualstudio.com/items?itemName=yzane.markdown-pdf) or the handy online tool [AnyConv](https://anyconv.com/md-to-pdf-converter/).**

**Note 2: This guide will constantly be updated with new info as becomes available and please feel to make an [issue](https://github.com/mikeroyal/Steam-Deck-Guide/issues) if you think something should be added.**

# Table of Contents

1. [Getting Started with the Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#getting-started-with-the-steam-deck)

     - [Steam Deck Accessories](https://github.com/mikeroyal/Steam-Deck-Guide#Steam-Deck-Accessories)
     - [Steam Deck Development](https://github.com/mikeroyal/Steam-Deck-Guide#steam-deck-development)
     - [SteamDB](https://github.com/mikeroyal/Steam-Deck-Guide#steamdb)
     - [Getting Software](https://github.com/mikeroyal/Steam-Deck-Guide#getting-software)
     - [Other Linux Operating Systems for the Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#Other-Linux-Operating-Systems-for-the-Steam-Deck)
     - [Getting Windows 10 or 11 on the Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#Getting-Windows-10-or-11-on-the-Steam-Deck)
     - [Improving Battery Life](https://github.com/mikeroyal/Steam-Deck-Guide#improving-battery-life)
     - [Tools to Copy/Transfer files to your Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#tools-to-copytransfer-files-to-your-steam-deck)
     - [Running Android Apps on your Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#running-android-apps-on-your-steam-deck)
     - [Running Bottles on your Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#running-bottles-on-your-steam-deck)
     - [Steam Tinker Launch](https://github.com/mikeroyal/Steam-Deck-Guide#steam-tinker-launch)
     - [RetroDECK](https://github.com/mikeroyal/Steam-Deck-Guide#RetroDECK)
     - [Adding Btrfs on Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#btrfs-on-steam-deck)
     - [Plugin Loaders for Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#plugin-loaders)
        * [Decky Loader](https://github.com/mikeroyal/Steam-Deck-Guide#plugin-loaders#decky-loader)
          * [Steam Deck Power Tools](https://github.com/mikeroyal/Steam-Deck-Guide#steam-deck-power-tools)
        * [Crankshaft](https://github.com/mikeroyal/Steam-Deck-Guide#plugin-loaders#crankshaft)
     - [Installing Unreal Engine 5 on your Steam Deck](https://github.com/mikeroyal/Steam-Deck-Guide#installing-unreal-engine-on-linux)

2. [Gaming](https://github.com/mikeroyal/Steam-Deck-Guide#gaming)

     - [Steam](https://github.com/mikeroyal/Steam-Deck-Guide#steam)
     - [ProtonDB](https://github.com/mikeroyal/Steam-Deck-Guide#protondb)
     - [Lutris](https://github.com/mikeroyal/Steam-Deck-Guide#lutris)
        * [Epic Games Store integration](https://github.com/mikeroyal/Steam-Deck-Guide#Epic-Games-Store-integration)
        * [Blizzard Battle.net integration](https://github.com/mikeroyal/Steam-Deck-Guide#blizzard-battlenet-intgeration)
        * [EA Play integration](https://github.com/mikeroyal/Steam-Deck-Guide#EA-Play-integration)
        * [Origin integration](https://github.com/mikeroyal/Steam-Deck-Guide#Origin-integration)
        * [Ubisoft Connect integration](https://github.com/mikeroyal/Steam-Deck-Guide#Ubisoft-Connect-integration)
        * [GOG Galaxy integration](https://github.com/mikeroyal/Steam-Deck-Guide#GOG-Galaxy-integration)
     - [GameHub](https://github.com/mikeroyal/Steam-Deck-Guide#gamehub)
     - [Epic Games Store](https://github.com/mikeroyal/Steam-Deck-Guide#epic-games-store)
     - [Game Streaming](https://github.com/mikeroyal/Steam-Deck-Guide#game-streaming)
     - [Game Emulators](https://github.com/mikeroyal/Steam-Deck-Guide#game-emulators)

3. [Game Development](https://github.com/mikeroyal/Steam-Deck-Guide#game-development)

4. [Vulkan Development](https://github.com/mikeroyal/Steam-Deck-Guide#vulkan-development)

5. [DirectX Development](https://github.com/mikeroyal/Steam-Deck-Guide#directx-development)

6. [OpenGL Development](https://github.com/mikeroyal/Steam-Deck-Guide#opengl-development)

7. [Wayland Development](https://github.com/mikeroyal/Steam-Deck-Guide#wayland-development)

8. [Audio Development](https://github.com/mikeroyal/Steam-Deck-Guide#audio-development)


# Getting Started with the Steam Deck
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Steam Deck](https://www.steamdeck.com/) is a handheld gaming computer developed by [Valve Corporation](https://valvesoftware.com/) in cooperation with [Advanced Micro Devices (AMD)](https://www.amd.com/). It allows users to play their entire Steam game library but can be modified by the user to run other gaming storefronts or applications. The Steam Deck started shipping in February 2022.

* [Steam Deck Booklet](https://store.steampowered.com/news/app/1675200/view/3401926123919972634)

* [Top Steam Deck Tips and Tricks Part 1 | GamingOnLinux](https://www.gamingonlinux.com/2022/08/top-quick-steam-deck-tips-and-tricks/)
 
* [Steam Deck Tips and Tricks Part 2 | GamingOnLinux](https://www.gamingonlinux.com/2022/08/steam-deck-tips-and-tricks-part-2/)

* [Steam Deck Teardown by Jeff Suovanen | iFixit](https://www.ifixit.com/News/57101/steam-deck-teardown)

* [Unlock Steam Deck tutorial | Chris Titus Tech](https://christitus.com/unlock-steam-deck/)

* [Steam Deck Hardware Review by Linus Tech Tips](https://www.youtube.com/watch?v=HjZ4POvk14c)

* [Steam Deck Unboxing Experience by Linus Tech Tips](https://www.youtube.com/watch?v=_UB9XoPlJ0U)

* [Steam Deck Tear Down by Linus Tech Tips](https://www.youtube.com/watch?v=ZK43RAc90ZA)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/142779563-30ada576-1bf4-42fb-8ad5-3fa3a6e40103.png">
<br />
</p>

Steam Deck device. Source: [Steam Deck](https://www.steamdeck.com/)

[Steam OS 3.0](https://store.steampowered.com/steamdeck) is an [immutable](https://en.wikipedia.org/wiki/Immutable_object) Operating System(OS) using the [KDE Plasma](https://kde.org/plasma-desktop) desktop. This allows you to install applications in containers using [Flatpak](https://flatpak.org/), rather than onto the root filesystem. This means not only that the installation of applications is isolated from the core filesystem, but also that the ability for malicious applications to compromise/break your system is significantly reduced.

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/157353163-6f5c4c1a-a89f-4ee5-9ffe-1d9f991c773c.png">
  <br />
  SteamOS 3.0 with KDE Plasma Desktop
</h3>

**Steam Deck Specs:**

Operating system: [SteamOS](https://en.wikipedia.org/wiki/SteamOS) 3.0 based on [ArchLinux](https://archlinux.org/).

System on a chip (SoC): AMD custom APU.

CPU: Zen 2, 4-core, 8-threads, variable frequency @ 2.4–3.5 GHz.

Memory:	16 GB LPDDR5 @ 5500 MT/s.

Storage Options:

 - Base model: 64 GB eMMC for $399(US).
 - Mid model: 256 GB NVMe SSD for $529(US).
 - Top model: 512 GB NVMe SSD for $649(US).

**All models use M.2 2230 interface.**

Removable Storage: **microSD** supports up to 2TB of additional storage for games.

Display: 7-inch, 1280 × 800 LCD, native 800p @ 60Hz.

Docked: Up to 8K @ 60Hz or 4K @ 120Hz.

Graphics: RDNA 2 with 8 CUs, variable frequency @ 1.0–1.6 GHz.

## Steam Deck Accessories
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[PROJECT Killswitch](https://dbrand.com/killswitch) is a custom protective case for the Steam Deck designed by [dbrand](https://dbrand.com/).

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/155908502-f015431e-6abd-4e31-8ad1-2d43f5ba6850.png">
<br />
 Front
</p>

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/155910314-78b454c8-94d3-43dc-a9e2-75b9819c9398.png">
<br />
 Back
</p>

[dbrand Skins](https://dbrand.com/shop/steam-deck-skins) Skins for the Steam Deck made by dbrand.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193435035-6b799061-f08d-4948-810f-59ca59a9be13.png">
<br />
 dbrand Skin
</p>

[Xtremeskins.co.uk Skins](https://www.xtremeskins.co.uk/products/steam-deck-skins) Skins for the Steam Deck made by Xtremeskins. Note that Xtremeskins is based in the UK. They ship globally, but it may take some time.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193435142-3012f99f-e3b3-4f9c-ba38-52c257058d7b.png">
<br />
 Xtremeskins skin
</p>

[Custom Skins](https://drive.google.com/file/d/1rQFoLlpMUKFEJcRIh80UvpmdVXZIsEZz/view) If you have access to the equipment it is possible to make your own skins from vinyl. This file provides the necessary cut template.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193435273-4f2c9363-6f03-4e91-b6aa-60628caffab6.png">
<br />
 Custom skins cut template
</p>

[JSAUX Protective Standing Case](https://www.jsaux.com/products/steam-deck-stand-protector) is a case for the Steam Deck made by JSAUX which features a kick stand.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193434666-38889d22-e17c-4183-9874-650dd4d41e92.png">
<br />
 JSAUX Protective Standing Case front and back view
</p>

[JSAUX Protective Case](https://www.jsaux.com/products/steam-deck-protective-case) is a case for the Steam Deck made by JSAUX.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193434692-46fad5ea-4f70-4c66-9f15-4b308f18d1df.png">
<br />
 JSAUX Protective Case
</p>

[Spigen Steam Deck Case](https://www.spigen.com/collections/steam-deck/products/steam-deck-case-rugged-armor?variant=41139474923567) is a case for the Steam Deck made by Spigen. It features a hard plastic element on the back which some individuals have used for modding.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193434798-16e3bc45-d5ca-4ba4-b4b9-c886ed395966.png">
<br />
 Spigen Case
</p>

[JSAUX Steam Deck Docking Station](https://www.jsaux.com/products/upgraded-docking-station-for-steam-deck) is a 6-in-1 Docking Station for Steam Deck. Equipped with an HDMI 4K@60Hz output, a Gigabit LAN Ethernet input, a USB-C port for charging, and three USB-A 3.0 ports, letting you explore a new way to play with Steam Deck. It supports 100W(Maximum) power delivery, which is enough power to charge your Steam Deck at full speed (45W) when paired with the original charger.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/186289763-9eca1e86-ed13-4cd4-a1ce-c4e433442d78.png">
<br />
JSAUX Steam Deck Docking Station
</p>

[JSAUX Steam Deck Docking Station With M.2](https://www.jsaux.com/products/m-2-docking-station-for-steam-deck-hb0604) is a 6-in-1 Docking Station for Steam Deck. Equipped with an HDMI 4K@60Hz output, a Gigabit LAN Ethernet input, a USB-C port for charging, two USB-A 3.0 ports, and a full size M.2 slot. It supports 100W(Maximum) power delivery, which is enough power to charge your Steam Deck at full speed (45W) when paired with the original charger. The M.2 slot supports 2230, 2242, 2260, or 2280 M.2 drives with M or M&B key at up to 900MB/s. The drive is automatically mounted when attached to the dock using a script provided by JSAUX.

<p align="center">
<img src="https://user-images.githubusercontent.com/88871218/193434546-dac68ed0-02d5-44d2-a1cf-7244d0792f74.png">
<br />
JSAUX Steam Deck Docking Station M.2
</p>

[NexDock](https://nexdock.com/features/) is an accessory with a HDMI-in port and Type-C cable NexDock that turns the Steam Deck into a fully functional computer.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/163284895-74fdfeaa-54df-4d7c-8e79-6117ca40b794.jpg">
<br />
</p>

**NexDock 2-in-1. Source: [NexDock](https://nexdock.com/features/)**

[Xbox Wireless Controller + USB-C® Cable](https://www.xbox.com/en-us/accessories/controllers/xbox-wireless-controller-usb-c)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/187094245-3c406751-4e4b-42fd-bd2c-a72181722fad.png">
<br />
Xbox Controller
</p>

[PlayStation 5 DualSense™ Wireless Controller](https://www.playstation.com/en-us/accessories/dualsense-wireless-controller/)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/187094347-109c80cd-5cc3-4a97-8a8f-0181687ab0d4.png">
<br />
PS 5  DualSense™ Controller
</p>

[Nintendo Switch Pro Controller](https://www.amazon.com/Nintendo-Switch-Pro-Controller/dp/B01NAWKYZ0)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/194023448-09e74efa-67f8-4503-87f5-5b7e59289608.png">
<br />
Nintendo Switch Pro Controller
</p>

[Glass Sceen Protector for Steam Deck](https://www.amazon.com/Tempered-Protector-Compatible-Installation-Manufacturer/dp/B09L9F65JG)

[M.2 2230 SSDs on Amazon](https://www.amazon.com/m-2-2230-ssd/s?k=m.2+2230+ssd)

[M.2 2230 SSDs on NewEgg](https://www.newegg.com/p/pl?d=m.2+2230+ssd)
 
[Lexar MicroSD cards on Amazon](https://www.amazon.com/lexar-micro-sd-cards/s?k=lexar+micro+sd+cards)

[SanDisk 1TB MicroSD Card on Amazon](https://www.amazon.com/SanDisk-1TB-Micro-SD-Cards/s?k=SanDisk+1TB+Micro+SD+Cards)

[Samsung 512GB EVO Select MicroSD Card on Amazon](https://www.amazon.com/SAMSUNG-Adapter-microSDXC-MB-ME512KA-AM/dp/B09B1HMJ9Z)

[Anker 45w Ultra Slim Charging PowerPort](https://www.amazon.com/Charger-Anker-Ultra-Slim-PowerPort-Laptops/dp/B0841J6CMK)

[Anker PowerCore Battery Bank](https://www.amazon.com/Anker-Portable-PowerCore-Essential-Compatible/dp/B08LG2X98F)

[Anker USB C Hub](https://www.anker.com/collections/hubs)


## Steam Deck Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

**Quick Links to Development Resources**

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

[AMD FidelityFX Super Resolution (FSR) 2.0](https://www.amd.com/en/press-releases/2022-03-17-introducing-amd-software-adrenalin-edition-2022-release-and-amd) is an open source, high-quality solution for producing high resolution frames from lower resolution inputs. It uses temporal data and optimized anti-aliasing to boost framerates in supported games while delivering similar or better image quality than native resolution without requiring dedicated machine learning hardware.

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

## SteamDB
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[SteamDB](https://steamdb.info/instantsearch/) is a database of everything on Steam. Important note SteamDB is a hobby project and is not affiliated with Valve or Steam.
   
   - [Steam Deck Verified Games List](https://steamdb.info/instantsearch/?refinementList%5Boslist%5D%5B0%5D=Steam%20Deck%20Verified)
   
   - [Steam Deck Playable Games List](https://steamdb.info/instantsearch/?refinementList%5Boslist%5D%5B0%5D=Steam%20Deck%20Playable)
   
   - [Steam Deck Unsupported Games List](https://steamdb.info/instantsearch/?refinementList%5Boslist%5D%5B0%5D=Steam%20Deck%20Unsupported)


## Getting Software
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Discover](https://apps.kde.org/discover/) is an software center that let's you manage software from multiple sources, including your operating system's software repository, Flatpak repos, the Snap store, or even AppImages from store.kde.org. Also, Discover allows you to find, install, and manage add-ons for Plasma and all your favorite KDE apps.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/156265563-1e9776f6-048f-4c20-b93b-d06ddcafed6d.png">
<br />
 KDE Plasma Discover App Store
</p>

[Flathub](https://flathub.org/) is an app store and build service for hundreds of apps which can be easily installed on any Linux distribution. Browse the apps online, from your app center or the command line.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/156265570-56fe8837-3963-49c8-b3f9-24a219929625.png">
<br />
 Flathub App Store
</p>

**Note:** Flathub app search will be integrated in Discover, if you want to limit the app search to Flathub you can mark Flatpak as default by clicking on the star.

[Flatseal](https://github.com/tchx84/flatseal) is a graphical utility to review and modify permissions from your [Flatpak](https://flatpak.org/) applications. [Get it on Flathub store](https://flathub.org/apps/details/com.github.tchx84.Flatseal).


## Other Linux Operating Systems for the Steam Deck.
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[HoloISO](https://github.com/theVakhovskeIsTaken/holoiso) is a SteamOS 3 (Holo) archiso configuration. It aims to bring the Steam Deck's Holo OS into a generic, installable format, and provide a close-to-official SteamOS experience.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/167318762-c54fffa2-9ed4-4695-9d7d-4d03eb5ba49d.png">
  <br />
</p>

HoloISO Desktop. Credit: [theVakhovskeIsTaken](https://github.com/theVakhovskeIsTaken/)

[Nobara Project](https://gitlab.com/GloriousEggroll/nobara-images) is an unofficial Fedora Linux Spin that's tailored for Gaming. It adds the necessary packages/tools (such as [Lutris](https://lutris.net/) and [ProtonUp-Qt](https://davidotek.github.io/protonup-qt/)), and fixes issues to make Fedora awesome for gaming. This project is developed and maintained by [Thomas Crider AKA Glorious Eggroll](https://gitlab.com/GloriousEggroll).

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/179671757-008ac6ef-ee95-43e9-b6eb-2f9bb928f91b.png">
  <br />
</p>

[WinesapOS](https://github.com/LukeShortCloud/winesapOS) is a project developed by [LukeShortCloud](https://github.com/LukeShortCloud) that provides an easy to setup installation of Linux. It can be used on a flash drive, SD card, HDD, SSD, NVMe, or any other storage device. The [release images](https://github.com/LukeShortCloud/winesapOS/releases) are based on SteamOS 3 and the KDE Plasma desktop environment to align with what Valve's [Steam Deck](https://store.steampowered.com/steamdeck/) uses.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/163284898-ca65b1ac-8ebc-4adc-b5aa-bd6b5195295e.jpg">
  <br />
</p>

WineapOS Desktop. Credit: [LukeShortCloud](https://github.com/LukeShortCloud)

 **[Manjaro Linux](https://manjaro.org/)**

 <h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779587-fbfac305-7cae-4768-80e8-d87830471232.png">
  <br />
  Manjaro Linux Desktop with KDE
</h3>

**[EndeavourOS](https://endeavouros.com/)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/107439882-9e414780-6ae7-11eb-819e-e87e7bcc7a97.png">
  <br />
  EndeavourOS Desktop
</h3>

**[Garuda Linux](https://garudalinux.org/)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/127042105-f6a6d97e-77bd-402e-af4f-df7af588eb08.png">
  <br />
 Garuda Linux Desktop
</h3>

**[ArcoLinux](https://arcolinux.com/)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/128940632-9e2a198f-f84d-490b-b4a2-22f6217ee574.png">
  <br />
ArcoLinux Desktop
</h3>

**[ArchTitus](https://github.com/ChrisTitusTech/ArchTitus) created by [ChrisTitusTech](https://www.youtube.com/channel/UCg6gPGh8HU2U01vaFCAsvmQ)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142780015-51242184-ff8b-4705-b6b3-f1913b734bf2.png">
  <br />
ArchTitus Desktop
</h3>

**[Fedora Linux](https://getfedora.org/)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779592-8b70c81e-ac10-4bb3-91b5-efe25fa9afb4.png">
  <br />
Fedora Desktop
</h3>

**[Pop!_OS](https://pop.system76.com)** created by [System76](https://system76.com).

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779593-390dfd58-a246-4299-baf2-adf0207da696.png">
  <br />
Pop!_OS Desktop
</h3>

**[Batocera](https://batocera.org/)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/4238928/163190916-d39124bb-c67e-42e4-a97c-dac78684c452.png">
  <br />
Emulation Station Front End
</h3>

## Getting Windows 10 or 11 on the Steam Deck
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

 **Useful YouTube videos:**

 * [Steam Deck Windows Install to a microSD | Wagner's TechTalk | YouTube](https://www.youtube.com/watch?v=pnpZboy_VQE)
 * [Windows On The Steam Deck: Benchmarks, Gaming, 4K | ETAPrime | YouTube](https://www.youtube.com/watch?v=vkqIjr4Ni6E)

 **[Steam Deck Windows Resources](https://help.steampowered.com/en/faqs/view/6121-ECCD-D643-BAA8)**

 **[Recovery instructions](https://help.steampowered.com/en/faqs/view/1B71-EDF2-EB6D-2BB3) for getting back to the default Steam Deck OS.**

**Creating a Windows 10/11 Bootable Device (MicroSD or USB)**

[Rufus](https://rufus.ie/) is a utility that helps format and create bootable USB flash drives.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/158471950-74640216-66ed-407b-a615-e643284ba0b8.png">
  <br />
  Rufus
</p>

**In Rufus 3.19:**

Add a new selection dialog for Windows 11 setup customization:

  * Secure Boot and TPM bypass have now been moved to this dialog.
  * Allows to bypass the mandatory requirement for a Microsoft account on Windows 11 22H2.
    **(Note: Network must be temporarily disabled for the local account creation to be proposed).**
  * Added an option to skip all collection questions (Sets all answers to “Don’t allow”).
  * Added an option for setting internal drives offline for Windows To Go.
  
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/183272077-015b8bb2-af94-443a-a455-f2018fcbd52a.png">
  <br />
  Rufus 3.19 Windows 11 setup customization.
</p>

**[Windows 11](https://www.microsoft.com/en-us/software-download/windows11)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/124997795-20cf2400-e000-11eb-8954-4944286b8ea8.png">
  Windows 11 Desktop
</h3>

**[Windows 10](https://www.microsoft.com/en-us/software-download/windows10ISO)**

<h3 align="center">
 <img src="https://user-images.githubusercontent.com/45159366/120387363-a9aabf80-c2de-11eb-84a5-8e4b422e7546.png">
  Windows 10 Desktop
</h3>

## Improving Battery Life
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Steam Deck - How To Get The Best Battery Life In Games! | The Phawx YouTube](https://www.youtube.com/watch?v=zB6tkjuXX8A-Y)

[auto-cpufreq](https://github.com/AdnanHodzic/auto-cpufreq) is an automatic CPU speed & power optimizer for Linux based on active monitoring of laptop's battery state, CPU usage, CPU temperature and system load. Ultimately allowing you to improve battery life without making any compromises. 
   
   - [auto-cpufreq - tool demo (Automatic CPU speed & power optimizer for Linux) | YouTube](https://www.youtube.com/watch?v=QkYRpVEEIlg-Y)

[Power-Control-Panel](https://github.com/project-sbc/Power-Control-Panel) is a software tool that allows easy change of TDP on **windows handheld devices(including Steam Deck)**. Features include: 

  - Configurable profiles with charger vs battery values -xinput TDP change capability (hold LB, RB, and a dpad direction) 
  - Assign an exe to a profile to auto start when exe opens 
  - Auto startup at logon to system tray 
  - Touch friendly interface, keyboard (mostly) not required. 
        
 [Power Control App | Project-SBC YouTube](https://www.youtube.com/watch?v=PcSV1tto2OM-Y)
 
## Tools to Copy/Transfer files to your Steam Deck
 [Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)
 
[KDE Connect](https://kdeconnect.kde.org/) is a tool that lets you easily link up your phone to your computer, your computer to your tablet; or your computer to your Steam Deck device. It can be used to share files across devices, read and send SMS directly from your laptop, and lock up your computer remotely.

[Warpinator](https://github.com/linuxmint/warpinator) is a free, open-source tool for sending and receiving files between computers that are on the same network. [Warpinator Flatpak](https://flathub.org/apps/details/org.x.Warpinator)

[FileZilla Client](https://filezilla-project.org/) is a fast and reliable cross-platform FTP, FTPS and SFTP client with lots of useful features and an intuitive graphical user interface. [FileZilla Flatpak](https://flathub.org/apps/details/org.filezillaproject.Filezilla)

[Dragit](https://github.com/sireliah/dragit) is an application for intuitive file sharing between devices. It's useful for when you want to send file from one computer to another with minimal effort. Dragit automatically detects devices in the local network with help of mDNS protocol and allows you to send file immediately. [Dragit Flatpak](https://flathub.org/apps/details/com.sireliah.Dragit)

[WinFsp](https://github.com/winfsp/winfsp) is a set of software components for Windows computers that allows the creation of user mode file systems. In this sense it is similar to FUSE (Filesystem in Userspace), which provides the same functionality on UNIX-like computers.

[SSHFS-Win](https://github.com/winfsp/sshfs-win) is a minimal port of SSHFS to Windows. Looking under the hood it uses Cygwin for the POSIX environment and WinFsp for the FUSE (Filesystem in Userspace) functionality.

[RiftShare](https://riftshare.app) is a cross platform (Windows, MacOS, Linux) file sharing tool that supports fully encrypted transfers both on the local network and off network using a simple passphrase. RiftShare uses [magic-wormhole](https://github.com/magic-wormhole/magic-wormhole) under the hood and is compatible with other magic-wormhole clients. It is also fully open source and licensed under the GPLv3. [RiftShare Flatpak](https://flathub.org/apps/details/app.riftshare.RiftShare)

[SyncThing](https://syncthing.net/) is a continuous file synchronization program. It synchronizes files between two or more computers in real time, safely protected from prying eyes. Works with Mac OS X, Windows, Linux, FreeBSD, Solaris, OpenBSD, Android, and many others. Downloadable in Discover. It's also open source and licensed under the MPL-2.0 license.

[Usermode FTP Server](https://gitlab.com/ergoithz/umftpd) is a tool that let's you start an FTP server as user and transfer files with any FTP client. Allowing you to access your files directly with many file browsers' builtin FTP support: Windows File Explorer, Thunar, Gnome Files, Dolphin and many more. [Usermode FTP Server on FlatHub](https://flathub.org/apps/details/eu.ithz.umftpd)

## Running Android Apps on your Steam Deck
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Waydroid](https://github.com/waydroid/waydroid) is a container-based approach to boot a full Android system on a regular Linux system. Make sure to checkout the [Waydroid Arch wiki page](https://wiki.archlinux.org/title/Waydroid). 

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/189516892-24454312-836a-4a88-a6b8-a3e9aaa8612c.png">
<br />
 
</p>

[Anbox](https://github.com/anbox) is a container-based software for running a full Android system on Linux distributions. Make sure to checkout the [Anbox Arch wiki page](https://wiki.archlinux.org/title/Anbox).


## Running Bottles on your Steam Deck
[Back to the Top](#table-of-contents)

[Bottles](https://usebottles.com/) is a software tool that let's you run Windows software on Linux. It's built-in dependency installation system grants automatic software compatibility access. The download manager can download the official components such as: the runner (Wine, Proton), DXVK, dependencies, etc. Available on [FlatHub](https://flathub.org/apps/details/com.usebottles.bottles) or throught [KDE Discover](https://apps.kde.org/discover/) store.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/182049940-ccba08e7-b05d-4991-b36f-1e2596c390da.png">
 </p>
 
 ## Steam Tinker Launch
 
 [Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)
 
 [Steam Tinker Launch](https://github.com/frostworx/steamtinkerlaunch) is a Linux wrapper tool for use with the Steam client which allows customizing and start tools and options for games quickly on the fly.
 
  <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/176962906-5d388e16-955e-4936-a676-c591d0937acd.png">
 </p>
 
 ## RetroDECK
 
 [Back to the Top](#table-of-contents)
 
[RetroDECK](https://retrodeck.net/) is a tool that brings an environment to catalog and play your retro games directly from SteamOS and it's tailored specifically for the Steam Deck. It's powered by EmulationStation Desktop Edition, which uses RetroArch and other standalone emulators to allow you to import and play your favorite retro (and even non-retro) games in a tidy environment without flooding your Steam library.
 
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/180627466-718dd292-cad0-48bb-93d0-e071a9e42194.png">
 </p>
 
 ## Btrfs on Steam Deck
 [Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)
 
 [SteamOS Btrfs](https://gitlab.com/popsulfr/steamos-btrfs/) is a project that will help get you from using ext4 on your Steam Deck's microSD card or home directory, to [Btrfs](https://btrfs.wiki.kernel.org/). 
 
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/172273657-f184233d-56d8-429b-9a63-d8a2b8e7412b.png">
 </p>
 
 ## Plugin Loaders
 [Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

 ### Decky Loader

[Decky Loader](https://github.com/SteamDeckHomebrew/decky-loader) is a plugin loader tool for the Steam Deck.

**Features:**

  - Clean injecting and loading of one or more plugins.
  - Persistent, meaning it doesn't need to be reinstalled after every system update.
  - Allows 2-way communication between the plugins and the loader.
  - Allows plugins to define python functions and run them from javascript.
  - Allows plugins to make fetch calls, bypassing cors completely.

A list of available plugins for Decky Loader can be found in [this store](https://plugins.deckbrew.xyz/).
 
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/176962910-9bf09ad0-1ab2-4524-bd50-7420afec2c4a.png">
 </p>

 #### Steam Deck Power Tools
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Steam Deck PowerTools](https://github.com/NGnius/PowerTools) is a plugin for Decky that provides system tweaks for power users.

   - Enable & disable CPU threads & SMT
   - Set CPU max frequency and toggle boost
   - Set some GPU power parameters (fastPPT & slowPPT)
   - Set the fan RPM (unsupported on SteamOS beta)
   - Display supplementary battery info
   - Keep settings between restarts (stored in ~/.config/powertools.json)
  
 <p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/168942613-685cf180-3b1d-4a29-ba10-f5bdfbcfdfb6.png">
 </p>

 ### Crankshaft
[Crankshaft](https://crankshaft.space/) is a Steam client plugin manager and framework. It lets you install and create plugins to add more functionality to your Steam client. [Available as a Flatpak](https://flathub.org/apps/details/space.crankshaft.Crankshaft)

 <p align="center">
 <img src="https://user-images.githubusercontent.com/2069735/193312550-4c3c5c0d-11ef-4eef-bf19-32e522398fd0.png">
 </p>

## Installing Unreal Engine on Linux

[Back to the Top](#table-of-contents)

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

**Quick links to Development Resources**

 - [Sign-up for Epic Games Acount](https://www.epicgames.com/account/password)
 
 - [Sign-up for Epic Games GitHub](https://github.com/EpicGames/Signup)
 
 - [Unreal Engine 5 Linux Binary Download](https://www.unrealengine.com/en-US/linux)
 
 - [Linux Development Requirements for Unreal Engine](https://docs.unrealengine.com/5.0/en-US/linux-development-requirements-for-unreal-engine/)
 
 - [Unreal Engine Performance and Profiling](https://docs.unrealengine.com/5.0/en-US/TestingAndOptimization/PerformanceAndProfiling/)
 
 - [Unreal Engine Blueprint API Reference](https://docs.unrealengine.com/5.0/en-US/BlueprintAPI/index.html)

 - [Unreal Engine C++ API Reference](https://docs.unrealengine.com/5.0/en-US/API/index.html)

 - [Unreal Engine Python API Reference](https://docs.unrealengine.com/5.0/en-US/PythonAPI/index.html)
 
# Gaming
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

## Steam

[Steam](https://store.steampowered.com/about/)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/106686402-13100100-657f-11eb-9012-6bdac264a808.png">
 </p>
 
[Steam Remote Play Together](https://store.steampowered.com/remoteplay/#together) is a steam service that let's you share your Steam local multi-player games with friends over the internet, for free. Using Remote Play Together, one player owns and runs the game, then up to four players can join.

[Steam Link app](https://store.steampowered.com/steamlink/about) is available free of charge, streaming your Steam PC games to phones, tablets, and TV.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/112692999-14ea9800-8e3d-11eb-964a-6bee4e665900.png">
</p>

[Proton](https://github.com/ValveSoftware/Proton/) is a tool for use with the Steam client which allows games which are exclusive to Windows to run on the Linux operating system. It uses Wine to facilitate this.

[ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt) is a tool to install and manage [Proton-GE](https://github.com/GloriousEggroll/proton-ge-custom) and [Luxtorpeda](https://github.com/luxtorpeda-dev/luxtorpeda) for Steam and [Wine-GE](https://github.com/GloriousEggroll/wine-ge-custom) for Lutris with this graphical user interface. Based on AUNaseef's [ProtonUp](https://github.com/AUNaseef/protonup), made with Python 3 and Qt 6.

## ProtonDB
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[ProtonDB](https://www.protondb.com) is a collection of over 100,000 gaming reports from other gamers as they test games with Proton on Linux and provide aggregate scores of how well games perform. A growing pool of suggestions provides tweaks that you can try to get games working while Proton continues development. In addition to this, you may explore the Steam game catalog on this site to browse and discover a wide range of titles that were previously unavailable for use on Linux.

 - [ProtonDB Anti-Cheat list for Games](https://www.protondb.com/explore?selectedFilters=antiCheat)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/108773213-dcd8f800-7512-11eb-8775-19b0c8924d55.png">
</p>

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/108773214-dd718e80-7512-11eb-983b-ce192e5b30f2.png">
</p>

## Lutris
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Lutris](https://lutris.net) is a gaming client for Linux. It gives you access to all your video games with the exception of the current console generation. Also, integrates nicely with other stores like GOG, Steam, Battle.net, Origin, Ubisoft Connect and many other sources that allow you to import your existing game library and community maintained install scripts give you a completely automated setup.

### Epic Games Store integration

[Back to the Top](#table-of-contents)

[Add Epic Games Store](https://lutris.net/games/epic-games-store/)

 <img src="https://user-images.githubusercontent.com/45159366/106686406-14412e00-657f-11eb-97c4-c80c6e25a374.png">
 
 ### Blizzard Battle.net intgeration
[Back to the Top](#table-of-contents)

[Blizzard Battle.net](https://lutris.net/games/battlenet/) is an internet-based online gaming, digital distribution, and digital rights management platform developed by Activision and Blizzard Entertainment. Battle.net is the launcher for World of Warcraft, Diablo III, StarCraft II, Hearthstone, Heroes of the Storm, Overwatch and Call of Duty.

<img src="https://user-images.githubusercontent.com/45159366/189614458-d51a15cb-d02d-4b1f-9e77-e712dcdb1d73.png">

### EA Play integration
[Back to the Top](#table-of-contents)

[EA Play](https://lutris.net/games/ea-desktop/) is a subscription-based video game service from Electronic Arts for the Xbox One, Xbox Series X/S, PlayStation 4, PlayStation 5 and Microsoft Windows platforms, offering access to selected games published by Electronic Arts along with additional incentives.

<img src="https://user-images.githubusercontent.com/45159366/189614466-476e0c4e-bab9-44bd-86c4-8aeadd739b63.png">

### Origin integration
[Back to the Top](#table-of-contents)
 
[Origin](https://lutris.net/games/origin/) is an online gaming, digital distribution and digital rights management (DRM) platform developed by Electronic Arts that allows users to purchase games on the internet for PC and mobile platforms, and download them with the Origin client (formerly EA Download Manager, EA Downloader and EA Link).

<img src="https://user-images.githubusercontent.com/45159366/189614468-49c4a05c-d6ca-4988-b3e6-10f0c71463d6.png">

### Ubisoft Connect integration
[Back to the Top](#table-of-contents)

[Ubisoft Connect](https://lutris.net/games/ubisoft-connect/) is a digital distribution, digital rights management, multiplayer and communications service created by Ubisoft to provide an experience similar to the achievements/trophies offered by various other game companies.

<img src="https://user-images.githubusercontent.com/45159366/189614471-422cbad8-1ae7-4f06-ad81-7f3b68550569.png">

### GOG Galaxy integration
[Back to the Top](#table-of-contents)

[GOG GALAXY](https://lutris.net/games/gog-galaxy/) is a fully optional client to install, play and update your games.

<img src="https://user-images.githubusercontent.com/45159366/189615528-385c01a8-f780-49e0-9502-db00d8082d9d.png"> 
        

## GameHub
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[GameHub](https://github.com/tkashkin/GameHub) is a unified library for all your games. It allows you to store your games from different platforms into one program to make it easier for you to manage your games.

<img src="https://user-images.githubusercontent.com/45159366/107862734-96451880-6e03-11eb-9b92-9d355b890083.png">

**GameHub supports:**

 - native games for Linux
 - **multiple compatibility layers:**
   - Wine
   - Proton
   - [DOSBox](https://www.dosbox.com/)
   - [RetroArch](https://store.steampowered.com/app/1118310/RetroArch/)
   - [ScummVM](https://www.scummvm.org/)
   - [WineWrap](https://www.gog.com/forum/general/adamhms_linux_wine_wrappers_news_faq_discussion/post1) — a set of preconfigured wrappers for [supported games](https://www.gog.com/forum/general/adamhms_linux_wine_wrappers_news_faq_discussion/post3);
   - custom emulators

 - **multiple game platforms:**
   - [Steam](https://store.steampowered.com/)
   - [GOG](https://www.gog.com/)
   - [Humble Bundle (including Humble Trove)](https://www.humblebundle.com/)
   - [itch.io](https://itch.io/)


## Epic Games Store
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Heroic](https://heroicgameslauncher.com/) is an Open Source Game Launcher for Linux, Windows and macOS (for both Native and Windows Games using Crossover). It supports launching games from the Epic Games Store using Legendary, a CLI alternative to the Epic Games Launcher. [Flatpak for Heroic Games Launcher](https://flathub.org/apps/details/com.heroicgameslauncher.hgl)

[Epic Games Store](https://www.epicgames.com/store/) is a digital video game storefront for Microsoft Windows and macOS, operated by Epic Games.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/111918016-3fed7a00-8a40-11eb-964e-930c801c1c72.png">
</p>

## Game Streaming
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[Geforce NOW](https://www.nvidia.com/en-us/geforce-now/download/) is NVIDIA's Cloud Gaming Service.

 <img src="https://user-images.githubusercontent.com/45159366/106686391-0f7c7a00-657f-11eb-9d0b-1ebb4d385883.jpeg">

[Moonlight Game Streaming](https://moonlight-stream.org/) is a program that let you stream from your PC games over the Internet with no configuration required. Stream from almost any device, whether you're in another room or miles away from your gaming rig. [Sunshine](https://github.com/LizardByte/Sunshine) is a **Game stream host for Moonlight** that is a self-hosted, low latency, cloud gaming solution with support for AMD, Intel, and NVIDIA GPUs. 

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/106686398-11463d80-657f-11eb-841a-d534829ccc3d.png">
</p>

[Chiaki](https://git.sr.ht/~thestr4ng3r/chiaki) is a Free and Open Source Software Client for PlayStation 4 and PlayStation 5 Remote Play for Linux, FreeBSD, OpenBSD, Android, macOS, Windows, Nintendo Switch and potentially even more platforms. [Chiaki Flatpak](https://flathub.org/apps/details/re.chiaki.Chiaki)

[Xbox Cloud Gaming](https://www.xbox.com/en-US/xbox-game-pass/cloud-gaming) is Microsoft's cloud-based Xbox game-streaming technology **(currently in Beta)**. **Play games like Forza Horizon 4, Halo 5: Guardians, Gears of War 4, Sea of Thieves, Cuphead, Red Dead Redemption 2, and 100+ other games on your mobile device or Chrome web browser**. Xbox Cloud Gaming does require an [Xbox Game Pass Ultimate](https://www.xbox.com/en-US/xbox-game-pass/cloud-gaming) subscription.

<img src="https://user-images.githubusercontent.com/45159366/108111388-74d56e00-7049-11eb-8aeb-3e5d65f9e832.png">

[Parsec](https://parsec.app/cloud-gaming) is a video game streaming platform, which offers a wide variety of games and genres to choose from and provides a high-quality and smooth gameplay. SParsec is developed in order to provide a high-quality smooth gameplay, same time to be free of all ads and in-game purchases.

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/166166858-e70ca081-8931-46f3-9dc3-fe9c719d76f8.png">
</p>

[Amazon Luna](https://www.amazon.com/luna/landing-page) is Amazon's Cloud Gaming Service. Amazon Luna is Compatible/Supported on a vartiey of [Devices and Browsers](https://www.amazon.com/gp/help/customer/display.html?nodeId=GUFHUSX8X324T4XE).

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/112693072-364b8400-8e3d-11eb-9df0-d58af7ac9c9c.png">
</p>

## Game Emulators
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

[EmuDeck](https://www.emudeck.com/) is a tool that takes care of everything for your retrogaming needs from RetroArch Configuration, Bezels, Gamepad Configuration for GameCube, Wii, Citra, SNES, etc. EmuDeck will even install EmulationStation Desktop Edition and carry over all their custom configurations and no need to configure ROM paths or anything. 

[EmulationStation Desktop Edition (ES-DE)](https://www.es-de.org/) is a frontend application for browsing and launching games from your multi-platform game collection. It's  available for Unix/Linux, macOS(M1 & Intel) and Windows.

[Pegasus](https://pegasus-frontend.org/) is a cross platform, customizable graphical frontend for launching emulators and managing your game library (especially retro games) and launching them from one place. It's focused on customizability, cross platform support (including embedded devices) and high performance.

[RetroPie](https://retropie.org.uk/) is a frontend for emulators that allows you to turn your Raspberry Pi, ODroid C1/C2, or PC into a retro-gaming machine. It builds upon Raspbian, [EmulationStation](https://github.com/Aloshi/EmulationStation), RetroArch and many other projects to enable you to play your favourite Arcade, home-console, and classic PC games with the minimum set-up.

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/153087555-e1bde100-6079-4089-a33d-804e29064789.png">
<br />
</p>

[RetroArch](https://www.retroarch.com/) is a frontend for emulators, game engines and media players. It enables you to run classic games on a wide range of computers and consoles through its slick graphical interface. Settings are also unified so configuration is done once and for all. [RetroArch Flatpak](https://flathub.org/apps/details/org.libretro.RetroArch)

[Cartridge](https://github.com/unclebacon-live/cartridge) is a self-hosted game library made with Laravel + Vue.js.

**Cartridge Features**

   - Scan for ROM files and match with IGDB game information
   - Serve ROM download links alongside game details
   - Manage access to library with user creation and permissions (WIP)
   - Allow users to request games (Planned)
   - Play select ROMs in-browser using JS emulators (Planned)
   - Track played and favorite games (even ones that aren't available for download) (Planned)

<p align="center">
<img src="https://user-images.githubusercontent.com/45159366/172274231-d691a850-1879-44fb-8fa0-08e549d7bb29.png">
<br />
 Cartridge UI
</p>

[Dolphin](https://dolphin-emu.org) is an emulator for two recent Nintendo video game consoles: the GameCube and the Wii. It allows PC gamers to enjoy games for these two consoles in full HD (1080p) with several enhancements: compatibility with all PC controllers, turbo speed, networked multiplayer, and even more. [Dolphin Flatpak](https://flathub.org/apps/details/org.DolphinEmu.dolphin-emu)

[Citra](https://citra-emu.org/) is an open-source emulator for the Nintendo 3DS capable of playing many of your favorite games. [Citra Flatpak](https://flathub.org/apps/details/org.citra_emu.citra)

[yuzu](https://yuzu-emu.org) is an experimental open-source emulator for the Nintendo Switch from the creators of Citra.[Yuzu Flatpak](https://flathub.org/apps/details/org.yuzu_emu.yuzu)

[m64p](https://m64p.github.io/) is a Nintendo 64 Emulator. It uses mupen64plus-gui, a brand new mupen64plus frontend written in Qt5. It supports all of the things you’d expect from a frontend (savestate management, pausing, screenshots). [m64p Flatpak](https://flathub.org/apps/details/io.github.m64p.m64p)

[DeSmuME](https://desmume.org/) is a Nintendo DS emulator. [DeSmuME Flatpak](https://flathub.org/apps/details/org.desmume.DeSmuME)

[Snes9x](https://www.snes9x.com/) is a portable, freeware Super Nintendo Entertainment System (SNES) emulator. [Snes9x Flatpak](https://flathub.org/apps/details/com.snes9x.Snes9x) 

[bsnes](https://github.com/bsnes-emu/bsnes) is a Super Nintendo (SNES) emulator focused on performance, features, and ease of use. [bsnes flatpak](https://flathub.org/apps/details/dev.bsnes.bsnes)

[mGBA](https://mgba.io/) is a new emulator for running Game Boy Advance games. It aims to be faster and more accurate than many existing Game Boy Advance emulators, as well as adding features that other emulators lack. [mGBA Flatpak](https://flathub.org/apps/details/io.mgba.mGBA)

[DOSBox](https://www.dosbox.com/) is an open-source DOS emulator which primarily focuses on running DOS Games.

[DOSBox Staging](https://github.com/dosbox-staging/dosbox-staging) is a full x86 CPU emulator (independent of host architecture), capable of running DOS programs that require real or protected mode. [DOSBox Staging Flatpak](https://flathub.org/apps/details/io.github.dosbox-staging)

[Flycast](https://github.com/flyinghead/flycast) is a multi-platform Sega Dreamcast, Naomi and Atomiswave emulator derived from reicast. [Flycast Flatpak](https://flathub.org/apps/details/org.flycast.Flycast)

[DuckStation](https://www.duckstation.org/) is an simulator/emulator of the Sony PlayStation 1 console, focusing on playability, speed, and long-term maintainability. [Available as a Flatpak on Flathub](https://flathub.org/apps/details/org.duckstation.DuckStation).

[PCSX2](https://pcsx2.net/) is a PlayStation 2 'emulator', a free program that tries to replicate the PlayStation 2 console to enable you to play PS2 games on your PC. [PCSX2 Flatpak](https://flathub.org/apps/details/net.pcsx2.PCSX2)

[RPCS3](https://rpcs3.net/) is an experimental open-source Sony PlayStation 3 emulator and debugger written in C++ for Windows and Linux. RPCS3 started development in May of 2011 by its founders DH and Hykem. The emulator is currently capable of running over 1800 commercial titles powered by Vulkan and OpenGL. [RPCS3 Flatpak](https://flathub.org/apps/details/net.rpcs3.RPCS3)

[MAME](https://www.mamedev.org/) is a Arcade Machine Emulator.

[xemu](https://xemu.app/) is an original Xbox emulator.

[Xenia](https://github.com/xenia-project/xenia) is an Xbox 360 Emulator.

**Also checkout these subreddits for more great Game Emulators recommendations**
  
   - [r/emulation](https://www.reddit.com/r/emulation/)
   - [r/emulations](https://www.reddit.com/r/emulators/)
   - [r/RetroArch](https://www.reddit.com/r/RetroArch/)
   - [r/RetroPie](https://www.reddit.com/r/RetroPie/)
   - [r/DolphinEmulator](https://www.reddit.com/r/DolphinEmulator/)
   - [r/Citra](https://www.reddit.com/r/Citra/)
   - [r/cemu](https://www.reddit.com/r/cemu/)
   - [r/yuzu](https://www.reddit.com/r/yuzu/)
   - [r/OpenEmu](https://www.reddit.com/r/OpenEmu/)
   - [r/MAME](https://www.reddit.com/r/MAME/)
   - [r/EmuDev](https://www.reddit.com/r/EmuDev/)
   - [r/Roms](https://www.reddit.com/r/Roms/)

# Game Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/97361059-45151700-185c-11eb-9d12-dae51c79eb8a.png">
  <br />
</p>

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/119279021-ea6b5000-bbdd-11eb-9f59-5251fc3ac751.png">
  <br />
</p>

## Game Engines

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

# Vulkan Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/129622224-8c4cca51-9200-4d70-9d16-2610d704713a.png">
  <br />
</p>

## Vulkan Learning Resources

[Vulkan®](https://www.khronos.org/vulkan/) is a modern cross-platform graphics and compute API that provides high-efficiency, cross-platform access to modern GPUs used in a wide variety of devices from PCs and consoles to mobile phones and embedded platforms. Vulkan is currently in development by the Khronos consortium.

[Khronos Group GitHub](https://github.com/KhronosGroup)

[Vulkan Documentation](https://github.com/KhronosGroup/Vulkan-Docs)

[HLSL to SPIR-V Feature Mapping Manual](https://github.com/microsoft/DirectXShaderCompiler/blob/master/docs/SPIR-V.rst)

[Vulkan GLSL Ray Tracing Emulator Tutorial](https://www.gsn-lib.org/docs/nodes/raytracing.php)

[Getting Started with Vulkan](https://vulkan-tutorial.com/)

[Vulkan Samples](https://github.com/KhronosGroup/Vulkan-Samples)

[Khronos Community Forums](https://community.khronos.org/)

## Vulkan Tools, Libraries, and Frameworks

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

# DirectX Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142779599-6a46ab05-c64a-48fe-a775-3e4f46e41f63.png">
  <br />
</p>

## DirectX Learning Resources

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

# OpenGL Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/131386211-f507b5d4-a3c9-4c21-aadd-2aa5bde94d1e.png">
  <br />
</p>

## OpenGL Learning Resources

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

# Wayland Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/104235197-79cf4e00-5409-11eb-97a6-a12f7bd8ad2a.png">
  <br />
</p>

## Wayland Learning Resources

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

# Audio Development
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

<p align="center">
 <img src="https://user-images.githubusercontent.com/45159366/142936394-b546784e-231a-4391-9dd8-c686e5a7eee9.png">
  <br />
</p>

## Audio Learning Resources

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

[PipeWire](https://pipewire.org) is a server and user space API to deal with multimedia pipelines. It provides a low-latency, graph based processing engine on top of audio and video devices that can be used to support the use cases currently handled by both pulseaudio and JACK. PipeWire was designed with a powerful security model that makes interacting with audio and video devices from containerized applications easy. Nodes in the graph can be implemented as separate processes, communicating with sockets and exchanging multimedia content using fd passing.

[WirePlumber](https://pipewire.pages.freedesktop.org/wireplumber/) is a modular session / policy manager for [PipeWire](https://pipewire.org/) and a GObject-based high-level library that wraps PipeWire’s API, providing convenience for writing the daemon’s modules as well as external tools for managing PipeWire. The WirePlumber daemon implements the session & policy management service. It follows a modular design, having plugins that implement the actual management functionality.

[Core API](https://docs.pipewire.org/group__api__pw__core.html) is used by all clients that need to communicate with the [PipeWire Daemon](https://docs.pipewire.org/page_daemon.html) and provides the necessary structs to interface with the daemon.

[Implementation API](https://docs.pipewire.org/group__api__pw__impl.html) is primarily used by the [PipeWire Daemon](https://docs.pipewire.org/page_daemon.html) itself but also by the [PipeWire Session Manager](https://docs.pipewire.org/page_session_manager.html) and modules/extensions that need to build objects in the graph.

[SPA (Simple Plugin API)](https://docs.pipewire.org/group__api__spa.html) is an extensible API to implement all kinds of plugins.

[GStreamer](https://gstreamer.freedesktop.org/) is a library for constructing graphs of media-handling components. The applications it supports range from simple Ogg/Vorbis playback, audio/video streaming to complex audio (mixing) and video (non-linear editing) processing. Applications can take advantage of advances in codec and filter technology transparently.

[Media Source Extensions (MSE)](https://www.w3.org/TR/media-source/) is a [W3C specification](https://github.com/w3c/media-source) that allows JavaScript to send byte streams to media codecs within Web browsers that support HTML5 video and audio. Also, this allows the implementation of client-side prefetching and buffering code for streaming media entirely in JavaScript.

[WebRTC](https://webrtc.org/) is an open-source project that adds real-time communication capabilities to your application that works on top of an open standard. It supports video, voice, and generic data to be sent between peers, allowing developers to build powerful voice- and video-communication solutions.

## Contribute

- [x] If would you like to contribute to this guide simply make a [Pull Request](https://github.com/mikeroyal/Steam-Deck-Guide/pulls).

## License
[Back to the Top](https://github.com/mikeroyal/Steam-Deck-Guide#table-of-contents)

Distributed under the [Creative Commons Attribution 4.0 International (CC BY 4.0) Public License](https://creativecommons.org/licenses/by/4.0/).

