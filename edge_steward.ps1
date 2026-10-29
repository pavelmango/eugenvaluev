# ============================================================================
# BING DESKTOP & EDGE ACCESSIBILITY STEWARD (CDP WebSocket Bridge)
# Connects directly to the user's visible Microsoft Edge instance via
# C:\Users\muse-sandbox\AppData\Local\Microsoft\Edge\User Data\DevToolsActivePort
# ============================================================================
param(
    [Parameter(Mandatory=$false)][string]$Action = "list",
    [Parameter(Mandatory=$false)][string]$TargetId = "",
    [Parameter(Mandatory=$false)][string]$Url = ""
)

$portFile = "$env:LOCALAPPDATA\Microsoft\Edge\User Data\DevToolsActivePort"
if (-not (Test-Path $portFile)) {
    Write-Error "DevToolsActivePort not found at $portFile"
    exit 1
}

$lines = Get-Content $portFile
$wsUri = "ws://127.0.0.1:$($lines[0])$($lines[1])"

function Invoke-CdpCommand([string]$Method, [hashtable]$Params) {
    $ws = New-Object System.Net.WebSockets.ClientWebSocket
    $ct = [System.Threading.CancellationToken]::None
    $ws.ConnectAsync([Uri]$wsUri, $ct).Wait()
    $payload = @{
        id = 1
        method = $Method
        params = $Params
    } | ConvertTo-Json -Compress -Depth 5
    $bytes = [System.Text.Encoding]::UTF8.GetBytes($payload)
    $ws.SendAsync((New-Object System.ArraySegment[byte] -ArgumentList (,$bytes)), [System.Net.WebSockets.WebSocketMessageType]::Text, $true, $ct).Wait()

    $buf = New-Object byte[] 131072
    $res = $ws.ReceiveAsync((New-Object System.ArraySegment[byte] -ArgumentList (,$buf)), $ct).Result
    $ws.CloseAsync([System.Net.WebSockets.WebSocketCloseStatus]::NormalClosure, "done", $ct).Wait()
    return ([System.Text.Encoding]::UTF8.GetString($buf, 0, $res.Count) | ConvertFrom-Json)
}

if ($Action -eq "list") {
    $resp = Invoke-CdpCommand -Method "Target.getTargets" -Params @{}
    $resp.result.targetInfos | Where-Object { $_.type -eq "page" } | Select-Object targetId, title, url | Format-Table -AutoSize
} elseif ($Action -eq "activate") {
    Invoke-CdpCommand -Method "Target.activateTarget" -Params @{ targetId = $TargetId } | ConvertTo-Json -Compress
} elseif ($Action -eq "open") {
    Invoke-CdpCommand -Method "Target.createTarget" -Params @{ url = $Url } | ConvertTo-Json -Compress
}

