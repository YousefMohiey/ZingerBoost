; ZingerBoost Inno Setup Script
#define MyAppName "ZingerBoost"
#define MyAppVersion "0.2.1"
#define MyAppPublisher "YousefMohiey"
#define MyAppURL "https://github.com/YousefMohiey/ZingerBoost"
#define MyAppExeName "zingerboost_flutter.exe"

[Setup]
AppId={{B8F4A3D2-1E5C-4F8B-9A2D-6C7E8F1A3B5D}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
OutputDir=installer
OutputBaseFilename=ZingerBoost_Setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"

[Files]
Source: "release\zingerboost_flutter.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "release\zingerboost_core.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "release\flutter_windows.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "release\window_manager_plugin.dll"; DestDir: "{app}"; Flags: ignoreversion
Source: "release\screen_retriever_plugin.dll"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "Launch ZingerBoost"; Flags: nowait postinstall skipifsilent
