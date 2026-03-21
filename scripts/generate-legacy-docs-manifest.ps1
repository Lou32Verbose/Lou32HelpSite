param(
    [string]$Root = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$docsDir = Join-Path $Root "docs"
$outDir = Join-Path $Root "migration"
$outFile = Join-Path $outDir "legacy-docs-manifest.csv"

if (-not (Test-Path $outDir)) {
    New-Item -ItemType Directory -Path $outDir | Out-Null
}

function New-ManifestRecord {
    param(
        [string]$SourceFile,
        [string]$Disposition,
        [string]$CanonicalCluster,
        [string]$DestinationTopic = "",
        [string]$DestinationSlug = "",
        [string]$PageType = "",
        [string]$Status = "",
        [string]$ReviewNotes = ""
    )

    [PSCustomObject]@{
        source_file       = $SourceFile
        disposition       = $Disposition
        canonical_cluster = $CanonicalCluster
        destination_topic = $DestinationTopic
        destination_slug  = $DestinationSlug
        page_type         = $PageType
        status            = $Status
        review_notes      = $ReviewNotes
    }
}

function Resolve-BaseRecord {
    param([string]$Name)

    $lower = $Name.ToLowerInvariant()

    switch -Regex ($lower) {
        'usingbitstransfer|bitspowershellcommands' {
            return New-ManifestRecord $Name "merge" "bits-transfer" "powershell/networking" "/powershell/networking/bits-transfer/" "recipe" "published" "Consolidate legacy BITS examples into the canonical transfer recipe."
        }
        'bookmarkletslist' {
            return New-ManifestRecord $Name "merge" "bookmarklet-essentials" "browser-web/bookmarklets" "/browser-web/bookmarklets/essentials/" "template" "published" "Fold legacy bookmarklet snippets into the curated essentials page."
        }
        'reg_disablefontsmoothing|disable-font-smoothing|disablefontsmoothing' {
            return New-ManifestRecord $Name "merge" "disable-font-smoothing" "windows/display" "/windows/display/disable-font-smoothing/" "recipe" "published" "Reuse the existing font smoothing recipe as the canonical destination."
        }
        'rclone_templaterclonecopycommandsfortaskscheduler' {
            return New-ManifestRecord $Name "merge" "rclone-task-scheduler-template" "backup-sync/rclone" "/backup-sync/rclone/task-scheduler-template/" "template" "published" "Move scheduler-ready command wrappers into the canonical task scheduler template."
        }
        'lou32rclonecommandlist|rclone_example|rclone_onedrivebestsettings|robocopy_template_backup' {
            return New-ManifestRecord $Name "merge" "rclone-provider-copy-examples" "backup-sync/rclone" "/backup-sync/rclone/provider-copy-examples/" "template" "published" "Merge provider-specific copy commands into one reusable sync reference."
        }
        'wget2?|downloadallurlsfromopendirectory|waybackdownloadindex' {
            return New-ManifestRecord $Name "merge" "wget-recursive-download" "cli-tools/wget" "/cli-tools/wget/recursive-download/" "reference" "published" "Keep wget mirroring and recursive download variants under the canonical wget reference."
        }
        'yt-dlp|gallery-dl' {
            return New-ManifestRecord $Name "merge" "yt-dlp-media-download-templates" "cli-tools/yt-dlp" "/cli-tools/yt-dlp/media-download-templates/" "template" "published" "Collect audio download, playlist, and output template notes into one media download page."
        }
        'githubcli' {
            return New-ManifestRecord $Name "migrate" "github-create-repo" "cli-tools/github" "/cli-tools/github/create-repo-from-local/" "recipe" "published" "Promote the CLI repo creation note into a standalone GitHub CLI recipe."
        }
        'exiftool' {
            return New-ManifestRecord $Name "migrate" "exiftool-one-line-reference" "cli-tools/exiftool" "/cli-tools/exiftool/one-line-reference/" "reference" "published" "Curate the exiftool one-liner list into a stable reference page."
        }
        'microsoft\.winget\.client|winget_' {
            return New-ManifestRecord $Name "merge" "winget-package-management-reference" "cli-tools/winget" "/cli-tools/winget/package-management-reference/" "reference" "published" "Combine winget packaging, return-code, and client notes into one package management reference."
        }
        'psprofile|get-moduleexamples|powerShellget|install-and-import|importaliasesfromcsv|historyto10000|transcript|alias-examples|enablewingettabcompletion' {
            return New-ManifestRecord $Name "merge" "powershell-console-profile-customization" "powershell/profiles" "/powershell/profiles/console-and-profile-customization/" "template" "published" "Consolidate profile snippets, module helpers, and transcript patterns into one console/profile page."
        }
        'pausemethods|write-host|breakaline' {
            return New-ManifestRecord $Name "merge" "powershell-pause-and-output-patterns" "powershell/syntax" "/powershell/syntax/pause-and-output-patterns/" "reference" "published" "Merge pause methods and console output formatting examples into one syntax reference."
        }
        'where-object|get-itemproperty|searchforcim|listallcimclasses|getciminstance|svchost|serviceswithpowershell|checkingforadminrights|systemarchitecture|getallprocessesandvirtualmemory|networkmonitoring|viewopenports|ntttcp|devicesanddrivers' {
            return New-ManifestRecord $Name "merge" "powershell-system-inspection-patterns" "powershell/querying" "/powershell/querying/system-inspection-patterns/" "reference" "published" "Group PowerShell query, CIM, process, and inspection snippets into one system inspection reference."
        }
        'deletefilesfromlist|converttabdelimitedcsvtocommadelimited|renameandchangefileextensions|getallurls_pscommand|converthextoascii|sessionpathenvironmentvariables|listallmscfiles|pwsh\.dll|writedirlisttotextfile' {
            return New-ManifestRecord $Name "merge" "powershell-file-and-text-recipes" "powershell/filesystem" "/powershell/filesystem/file-and-text-recipes/" "recipe" "published" "Collect file, text, and path manipulation helpers into one recipe page."
        }
        'disablewindowsrecall' {
            return New-ManifestRecord $Name "migrate" "windows-disable-recall" "windows/policy" "/windows/policy/disable-windows-recall/" "recipe" "published" "Promote the Recall registry steps into a canonical Windows policy recipe."
        }
        'localgrouppolicyonwin11home' {
            return New-ManifestRecord $Name "migrate" "windows-local-group-policy-home" "windows/policy" "/windows/policy/local-group-policy-editor-on-home/" "recipe" "published" "Normalize the DISM-based Local Group Policy install steps into one Windows Home page."
        }
        'howtocreatesettingsappshortcuts|shellcommandslist|clsid_shelllocations|usingmsshelldlg|windowshelpclsidregistrylocation|regref_showlnkext|maponedriveasanetworklocation' {
            return New-ManifestRecord $Name "merge" "windows-settings-shell-shortcuts" "windows/shortcuts" "/windows/shortcuts/settings-and-shell-shortcuts/" "reference" "published" "Consolidate Settings URIs, shell namespace shortcuts, and shell folder references into one shortcut guide."
        }
        'instructionstobypasswindows11accountcreation|autounattend|bypassnro|directdownloadlinkforwindows11iso|genericproductkeys|changeproductkey|win11bypassrequirements|installwindowsfromcmd|win10enterprise_bootableiso|rufus_|allgenerickeysforwin11|visualstudio2015_downloadurls' {
            return New-ManifestRecord $Name "merge" "windows-install-and-oobe-notes" "windows/install" "/windows/install/windows-install-and-oobe-notes/" "recipe" "published" "Group install-media, OOBE, and setup notes into one curated Windows install page."
        }
        'sfc|cbslog|windowsstoreapps|appx|optionalfeature|dism|provisionedpackage|mountawindowsimage|makeprovisionedpackagelist|repaircorruptedsystemfiles|disablemalicioussoftwareremovaltool|changepagefilesettings' {
            return New-ManifestRecord $Name "merge" "windows-dism-appx-system-repair" "windows/maintenance" "/windows/maintenance/dism-appx-and-system-repair/" "recipe" "published" "Merge DISM, AppX, SFC, and maintenance commands into one repair-oriented workflow page."
        }
        'tabletmode|convertibilitycontrol|sensormonitoringservice|lenovoflex5|brightnessregkeyfix' {
            return New-ManifestRecord $Name "merge" "windows-lenovo-tablet-sensor-fixes" "windows/troubleshooting" "/windows/troubleshooting/lenovo-tablet-mode-and-sensor-fixes/" "troubleshooting" "published" "Consolidate Lenovo convertibility, tablet mode, and sensor-related fixes into one troubleshooting page."
        }
        'getscreendpi|dpisettingsforcreatingcustomstartmenu|settingconsolefonts|fontrenderingguide|fonts_|minimumneededfontlist|fontsubstitutes|rebuildfontcache|removefontsfromsystemreservedpartition' {
            return New-ManifestRecord $Name "merge" "windows-display-diagnostics" "windows/display" "/windows/display/display-diagnostics/" "reference" "published" "Group display diagnostics, font tuning, and DPI notes into one Windows display reference."
        }
        'googlesearchoperatorsreference|googleoperators|msedgedefaultsearchprovidergoogle|ungoogledchromiumaddgoogleassearchengine' {
            return New-ManifestRecord $Name "merge" "browser-search-operator-reference" "browser-web/search" "/browser-web/search/search-operator-reference/" "reference" "published" "Keep benign search operator and browser search-provider notes in the browser search reference."
        }
        'googledork|githubsearchdorks|googledorking' {
            return New-ManifestRecord $Name "merge" "defensive-search-audit-reference" "security/search" "/security/search/defensive-search-audit-reference/" "reference" "draft" "Rewrite search/dork material as defensive audit guidance and keep it in draft status."
        }
        'devtoolssecret|geturlsofallopentabs|chrome_chromiumbrowsers_devtoolsconsolemethod' {
            return New-ManifestRecord $Name "merge" "browser-devtools-and-tab-helpers" "browser-web/chromium" "/browser-web/chromium/devtools-and-tab-helpers/" "reference" "published" "Group DevTools resources and Chromium tab helpers into one browser debugging page."
        }
        'ffmpeg|imagemagick_automaticallygeneratefavicon' {
            return New-ManifestRecord $Name "merge" "ffmpeg-conversion-frame-extraction" "media/ffmpeg" "/media/ffmpeg/conversion-and-frame-extraction/" "template" "published" "Consolidate ffmpeg conversion, extraction, and related media command templates into one page."
        }
        'audio-eq-cookbook' {
            return New-ManifestRecord $Name "migrate" "biquad-eq-cookbook-reference" "media/audio" "/media/audio/biquad-eq-cookbook-reference/" "reference" "draft" "Retain the RBJ biquad formula notes as a draft technical reference pending deeper editorial cleanup."
        }
        'midi|gmmap|midiplayer' {
            return New-ManifestRecord $Name "merge" "core-midi-reference" "midi/reference" "/midi/reference/core-midi-reference/" "reference" "published" "Fold MIDI basics, terminology, patch maps, and charts into one core MIDI reference."
        }
        'ascii|colorcode|colorhex|gimpcolorpalette' {
            return New-ManifestRecord $Name "merge" "ascii-and-color-reference" "developer-tools/reference" "/developer-tools/reference/ascii-and-color-reference/" "reference" "published" "Group ASCII tables, hex values, and palette snippets into one developer reference page."
        }
        'imagebb_api|imgbb' {
            return New-ManifestRecord $Name "migrate" "imgbb-upload-overview" "developer-tools/apis" "/developer-tools/apis/imgbb-upload-overview/" "reference" "draft" "Convert the ImageBB upload note into a draft API reference with cleaned examples."
        }
        'registryvaluestomonitorformalware|fileinfectormalware|security_' {
            return New-ManifestRecord $Name "merge" "windows-registry-monitoring-reference" "security/windows" "/security/windows/registry-monitoring-reference/" "reference" "draft" "Keep defensive Windows monitoring content in a draft security reference."
        }
        'windowserrorcodes_master|winpeoptionalcomponents|winpecustomizableimageguide' {
            return New-ManifestRecord $Name "split" "windows-large-reference-batch" "windows" "" "reference" "" "Large legacy table or WinPE reference that should stay archived until a dedicated split-and-rewrite batch."
        }
        'chatgptconfigprompts|eulaviolation|productkey_windows2000professional' {
            return New-ManifestRecord $Name "archive" "legacy-archive" "" "" "" "" "Leave low-value, risky, or non-library material in the raw archive."
        }
        default {
            return New-ManifestRecord $Name "archive" "legacy-archive" "" "" "" "" "Leave this file in docs/ until a later migration batch rewrites or consolidates it."
        }
    }
}

$exactDuplicates = @{
    "getscreendpi.txt"                                   = "LOU32HELP_getscreendpi.txt"
    "HELP32_PSProfileCustomizations.txt"                 = "LOU32HELP_PSProfileCustomizations.txt"
    "ffmpeg templates_louis32.txt"                       = "LOU32HELP_REF_ffmpegtemplates.txt"
    "HELP32_Get-ModuleExamples.txt"                      = "LOU32HELP_Get-ModuleExamples.txt"
    "LOU32HELP_7DeToolssecretsthatshouldntbesecrets.txt" = "LOU32HELP_7DevToolssecretsthatshouldntbesecrets.txt"
    "LOU32HELP_wget2_wgetrecursivedownload_examples.txt" = "LOU32HELP_wgetrecursivedownload_examples.txt"
    "localgrouppolicyonwin11home.txt"                    = "LOU32HELP_localgrouppolicyonwin11home.txt"
}

$records = foreach ($file in (Get-ChildItem $docsDir -File | Sort-Object Name)) {
    $name = $file.Name

    if ($exactDuplicates.ContainsKey($name)) {
        $canonical = $exactDuplicates[$name]
        $base = Resolve-BaseRecord $canonical
        New-ManifestRecord `
            -SourceFile $name `
            -Disposition "merge" `
            -CanonicalCluster $base.canonical_cluster `
            -DestinationTopic $base.destination_topic `
            -DestinationSlug $base.destination_slug `
            -PageType $base.page_type `
            -Status $base.status `
            -ReviewNotes ("Exact duplicate of {0}; treat as an alias-only inventory record." -f $canonical)
        continue
    }

    Resolve-BaseRecord $name
}

$records | Export-Csv -Path $outFile -NoTypeInformation -Encoding UTF8
Write-Output ("Wrote {0} manifest rows to {1}" -f $records.Count, $outFile)
