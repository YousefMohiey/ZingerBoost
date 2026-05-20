Add-Type -AssemblyName System.Drawing

$iconPath = "C:\Users\Yousef-Laptop\ZingerBoost\crates\zb_app\icons\icon.png"
$icoPath = "C:\Users\Yousef-Laptop\ZingerBoost\crates\zb_app\icons\icon.ico"

$size = 512
$bmp = New-Object System.Drawing.Bitmap($size, $size)
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic

# Black background with rounded corners (like iOS app icon)
$bgBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(0, 0, 0))
$cornerRadius = 110
$g.FillPie($bgBrush, 0, 0, $cornerRadius*2, $cornerRadius*2, 180, 90)
$g.FillPie($bgBrush, $size-$cornerRadius*2, 0, $cornerRadius*2, $cornerRadius*2, 270, 90)
$g.FillPie($bgBrush, 0, $size-$cornerRadius*2, $cornerRadius*2, $cornerRadius*2, 90, 90)
$g.FillPie($bgBrush, $size-$cornerRadius*2, $size-$cornerRadius*2, $cornerRadius*2, $cornerRadius*2, 0, 90)
$g.FillRectangle($bgBrush, $cornerRadius, 0, $size-$cornerRadius*2, $size)
$g.FillRectangle($bgBrush, 0, $cornerRadius, $size, $size-$cornerRadius*2)

# White Z with lightning bolt - exact shape from user's image
$whiteBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(255, 255, 255))

$zPath = New-Object System.Drawing.Drawing2D.GraphicsPath

# The Z shape with lightning bolt cut
# Top horizontal bar (slightly angled)
$zPath.AddPolygon(@(
    (New-Object System.Drawing.Point(100, 120)),
    (New-Object System.Drawing.Point(380, 120)),
    (New-Object System.Drawing.Point(360, 160)),
    (New-Object System.Drawing.Point(180, 160)),
    (New-Object System.Drawing.Point(120, 240)),
    (New-Object System.Drawing.Point(260, 240)),
    (New-Object System.Drawing.Point(200, 320)),
    (New-Object System.Drawing.Point(140, 320)),
    (New-Object System.Drawing.Point(80, 400)),
    (New-Object System.Drawing.Point(400, 400)),
    (New-Object System.Drawing.Point(420, 360)),
    (New-Object System.Drawing.Point(180, 360)),
    (New-Object System.Drawing.Point(240, 280)),
    (New-Object System.Drawing.Point(420, 280)),
    (New-Object System.Drawing.Point(440, 240)),
    (New-Object System.Drawing.Point(260, 240)),
    (New-Object System.Drawing.Point(320, 160))
))

$g.FillPath($whiteBrush, $zPath)

$bmp.Save($iconPath, [System.Drawing.Imaging.ImageFormat]::Png)

$g.Dispose()
$bmp.Dispose()
$bgBrush.Dispose()
$whiteBrush.Dispose()
$zPath.Dispose()

Write-Host "Icon PNG created: $iconPath"

# Create ICO with multiple sizes
$png = [System.Drawing.Image]::FromFile($iconPath)
$icon = [System.Drawing.Icon]::FromHandle($png.GetThumbnailImage(256, 256, $null, [System.IntPtr]::Zero).GetHicon())
$fs = New-Object System.IO.FileStream($icoPath, [System.IO.FileMode]::Create)
$icon.Save($fs)
$fs.Close()
$icon.Dispose()
$png.Dispose()

Write-Host "Icon ICO created: $icoPath"

# Copy to other sizes
Copy-Item $iconPath "C:\Users\Yousef-Laptop\ZingerBoost\crates\zb_app\icons\128x128.png" -Force
Copy-Item $iconPath "C:\Users\Yousef-Laptop\ZingerBoost\crates\zb_app\icons\128x128@2x.png" -Force
Copy-Item $iconPath "C:\Users\Yousef-Laptop\ZingerBoost\crates\zb_app\icons\32x32.png" -Force

Write-Host "Icon files copied"
