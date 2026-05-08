# ZingerBoost — Build Instructions (Windows)

## One-Time Setup

### 1. Install Flutter
Open **PowerShell as Administrator** and run:
```powershell
winget install Google.Flutter
```
Close PowerShell and reopen it after installation completes.

### 2. Verify Flutter
```powershell
flutter --version
```
If this shows a version number, Flutter is ready. If not, restart your PC and try again.

### 3. Install Visual Studio Build Tools
If you don't have Visual Studio 2022, download the free **Build Tools** from:
https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022

During installation, select **Desktop development with C++** workload. This gives you the C++ compiler Flutter needs.

---

## Build ZingerBoost

### 4. Clone or pull the latest code
```powershell
cd C:\
git clone https://github.com/YousefMohiey/ZingerBoost.git
cd ZingerBoost
```

### 5. Go to the Flutter folder
```powershell
cd zingerboost_flutter
```

### 6. Generate Windows platform files
```powershell
flutter create --platforms windows .
```
**Important:** The dot `.` at the end means "this folder". You MUST be inside `zingerboost_flutter` when you run this.

### 7. Install Flutter packages
```powershell
flutter pub get
```

### 8. Build the app
```powershell
flutter build windows
```
This takes 3-5 minutes. The output will be at:
```
zingerboost_flutter\build\windows\x64\runner\Release\
```

### 9. Run the app
```powershell
.\build\windows\x64\runner\Release\zingerboost.exe
```

Or just go to that folder and double-click `zingerboost.exe`.

---

## If Something Goes Wrong

| Error | Fix |
|-------|-----|
| `flutter: command not found` | Restart your PC after installing Flutter |
| `Visual Studio not found` | Install Visual Studio Build Tools (step 3 above) |
| `MSVC compiler not found` | Select "Desktop development with C++" during VS install |
| `Missing DLL` when running | Install [VC++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe) |
| `flutter create` says project already exists | That's fine, it only adds missing files |

---

## Quick Rebuild After Changes

```powershell
cd C:\ZingerBoost\zingerboost_flutter
flutter pub get
flutter build windows
```
