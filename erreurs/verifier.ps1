# Compile chaque extrait de erreurs.txt et compare le code d'erreur obtenu au code attendu.
# N'a pas besoin de l'éditeur de liens : --emit=metadata s'arrête après la vérification.
$ErrorActionPreference = 'Continue'
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
$ici = Split-Path -Parent $MyInvocation.MyCommand.Path
$tmp = Join-Path $ici 'tmp'
New-Item -ItemType Directory -Force $tmp | Out-Null
$ok = 0; $total = 0
foreach ($ligne in [System.IO.File]::ReadAllLines((Join-Path $ici 'erreurs.txt'), [System.Text.Encoding]::UTF8)) {
    if ($ligne.Trim() -eq '' -or $ligne.StartsWith('#')) { continue }
    $parts = $ligne.Split('|', 3)
    $nom = $parts[0].Trim(); $attendu = $parts[1].Trim(); $src = $parts[2].Trim().Replace('\n', "`n")
    $fichier = Join-Path $tmp "$nom.rs"
    [System.IO.File]::WriteAllText($fichier, $src, (New-Object System.Text.UTF8Encoding $false))
    $sortie = cmd /c "rustc --edition 2024 --crate-type bin --emit=metadata --out-dir `"$tmp`" `"$fichier`" 2>&1" | Out-String
    $codes = [regex]::Matches($sortie, 'error\[(E\d+)\]') | ForEach-Object { $_.Groups[1].Value } | Select-Object -Unique
    $obtenu = if ($codes) { ($codes -join ',') } elseif ($sortie -match '(?m)^error') { 'erreur sans code' } else { 'OK' }
    $total++
    $bon = $false
    if ($attendu -eq 'OK') { $bon = ($obtenu -eq 'OK') }
    else { foreach ($a in $attendu.Split(',')) { if ($codes -contains $a.Trim()) { $bon = $true } } }
    if ($bon) { $ok++; "PASS  {0,-28} attendu {1,-12} obtenu {2}" -f $nom, $attendu, $obtenu }
    else { "FAIL  {0,-28} attendu {1,-12} obtenu {2}" -f $nom, $attendu, $obtenu; ($sortie -split "`n" | Select-Object -First 12) -join "`n" }
}
"$ok / $total extraits conformes"
cmd /c "rmdir /s /q `"$tmp`"" | Out-Null
