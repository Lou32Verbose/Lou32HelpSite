---
title: DNS And Network Monitoring
slug: /powershell/networking/dns-and-network-monitoring/
summary: PowerShell and command-line tools for network traffic monitoring, DNS configuration, open port scanning, TLS enablement, and NTTTCP benchmarking.
topic: powershell/networking
type: reference
tags: [powershell, networking, dns, netstat, tls, ntttcp, monitoring]
aliases: [network monitoring powershell win11, set dns over https, dns providers list, view open ports netstat, enable tls 1.2, ntttcp tool]
platforms: [windows, powershell]
related:
  - /powershell/networking/bits-transfer/
  - /powershell/querying/service-and-system-admin/
status: published
updated: 2026-03-21
---

## Synopsis

Reference for PowerShell network monitoring commands, DNS-over-HTTPS configuration, free DNS provider addresses, open port inspection with netstat, TLS 1.2 enablement, and NTTTCP network benchmarking.

## Syntax

```powershell
Get-NetAdapterStatistics -Name "Ethernet"
Get-NetTCPConnection | Sort-Object State
Set-DnsClientServerAddress -InterfaceAlias "<name>" -ServerAddresses <ip1>,<ip2>
```

## Parameters/Flags

- `-Name`: network adapter name for statistics commands
- `-InterfaceAlias`: adapter alias for DNS configuration
- `-State Established`: filter for active TCP connections
- `netstat -ano`: show all connections with PIDs
- `netstat -ab`: show connections with executable names

## Examples

### Network Adapter Traffic Statistics

```powershell
Get-NetAdapterStatistics -Name "Ethernet"
```

### Real-Time Traffic Monitoring

```powershell
while ($true) {
    Get-NetAdapterStatistics -Name "Ethernet"
    Start-Sleep -Seconds 2
}
```

### Track Active TCP Connections

```powershell
Get-NetTCPConnection | Sort-Object State | Format-Table

# Established connections only
Get-NetTCPConnection -State Established | Format-Table
```

### Measure Bandwidth Over Time

```powershell
$before = Get-NetAdapterStatistics -Name "Ethernet"
Start-Sleep -Seconds 10
$after = Get-NetAdapterStatistics -Name "Ethernet"
$bytesReceived = $after.ReceivedBytes - $before.ReceivedBytes
$bytesSent = $after.SentBytes - $before.SentBytes
Write-Output "Bytes received: $bytesReceived"
Write-Output "Bytes sent: $bytesSent"
```

### Check Adapter Link Speed

```powershell
Get-NetAdapter | Select-Object Name, LinkSpeed
```

### Monitor Listening Ports

```powershell
Get-NetTCPConnection | Where-Object { $_.State -eq 'Listen' } |
  Format-Table LocalPort, LocalAddress
```

### Automate Daily Network Traffic Reports

```powershell
$trigger = New-ScheduledTaskTrigger -Daily -At 6:00AM
$action = New-ScheduledTaskAction -Execute 'Powershell.exe' `
    -Argument 'Get-NetAdapterStatistics -Name "Ethernet" | Out-File -FilePath C:\NetworkLogs\DailyTraffic.txt'
Register-ScheduledTask -Action $action -Trigger $trigger -TaskName "Daily Network Traffic Monitoring"
```

### Set DNS-over-HTTPS (Cloudflare)

Find active adapter names:

```powershell
Get-NetAdapter | Where-Object Status -eq Up | Select-Object Name, IfIndex, Status
```

Set IPv4 DNS:

```powershell
Set-DnsClientServerAddress -InterfaceAlias "Wi-Fi" -ServerAddresses 1.1.1.1, 1.0.0.1
```

Set IPv6 DNS:

```powershell
Set-DnsClientServerAddress -InterfaceAlias "Wi-Fi" -ServerAddresses 2606:4700:4700::1111, 2606:4700:4700::1001 -AddressFamily IPv6
```

### Free DNS Provider Reference

| Provider | Primary | Secondary |
|----------|---------|-----------|
| Google Public DNS | `8.8.8.8` | `8.8.4.4` |
| Cloudflare DNS | `1.1.1.1` | `1.0.0.1` |
| OpenDNS Home | `208.67.222.222` | `208.67.220.220` |
| Quad9 | `9.9.9.9` | `149.112.112.112` |
| CleanBrowsing Security | `185.228.168.9` | `185.228.169.9` |
| AdGuard DNS | `94.140.14.14` | `94.140.15.15` |
| NextDNS (free tier) | `45.90.28.0` | `45.90.28.255` |
| OpenNIC | `192.95.54.3` | `192.95.54.1` |

### View Open Ports with netstat

```text
netstat -a
netstat -ano
netstat -ab
```

- `Local Address`: IP and port (number after the colon)
- `State`: `LISTENING` indicates an open port
- `PID`: process ID (with `-ano`)

Identify the application for a specific PID:

```text
tasklist /FI "PID eq 1234"
```

### Enable TLS 1.2

```powershell
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
```

### NTTTCP Network Benchmarking

**Receiver side:**

```text
ntttcp.exe -r -m 4,*,192.168.242.5 -l 64k -a 2 -t 30
```

**Sender side:**

```text
ntttcp.exe -s -m 4,*,192.168.242.5 -l 64k -a 2 -t 30
```

Both commands use the receiver's IP address. The tool runs asynchronously for 30 seconds using four threads, two I/O buffers, and 64K buffer size. Output can be saved as XML for analysis.

## Related

- [`BITS Transfer`](/powershell/networking/bits-transfer/)
- [`PowerShell Service And System Administration`](/powershell/querying/service-and-system-admin/)
