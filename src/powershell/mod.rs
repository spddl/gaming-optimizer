use powershell_script;

pub fn check_bcd_store() {
    let command = r#"
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope Process

[uint32]$useplatformclock = '0x260000a2'
[uint32]$useplatformtick = '0x260000a4'
[uint32]$disabledynamictick = '0x260000a5'
[uint32]$resumeobject = '0x23000003'

$BcdStore = (Invoke-CimMethod -ClassName BcdStore -Arguments @{File = ([System.String]::Empty)} -MethodName OpenStore -Namespace root\wmi).Store
$BcdBootMgrObject = (Invoke-CimMethod -Arguments @{Id = '{9dea862c-5cdd-4e70-acc1-f32b344d4795}'} -MethodName OpenObject -InputObject $BcdStore).Object
$BcdGuid = (Invoke-CimMethod -InputObject $BcdBootMgrObject -MethodName GetElement -Arguments @{Type = $resumeobject}).Element.Id
$BcdObject = (Invoke-CimMethod -Arguments @{Id = $BcdGuid} -MethodName OpenObject -InputObject $BcdStore).Object

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformclock} -ErrorAction Stop
  if ($result.Element.Boolean) {
    [Console]::Error.WriteLine('wrong setting: useplatformclock Yes')
  } else {
    Write-Host 'correct setting: useplatformclock No'
  }
} catch {
  [Console]::Error.WriteLine('setting missing: useplatformclock None')
}

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $disabledynamictick} -ErrorAction Stop
  if ($result.Element.Boolean) {
    Write-Host 'correct setting: disabledynamictick Yes'
  } else {
    [Console]::Error.WriteLine('wrong setting: disabledynamictick No')
  }
} catch {
  [Console]::Error.WriteLine('setting missing: disabledynamictick None')
}

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformtick} -ErrorAction Stop
  if ($result.Element.Boolean) {
    Write-Host 'correct setting: useplatformtick Yes'
  } else {
    [Console]::Error.WriteLine('wrong setting: useplatformtick No')
  }
} catch {
  [Console]::Error.WriteLine('setting missing: useplatformtick None')
}

"#;
    // powershell.exe -ExecutionPolicy Bypass -File C:\MyUnsignedScript.ps1
    match powershell_script::run(command, false) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("\x1b[0;93m{}\x1b[0m ", e);
        }
    }
}

// TODO: aber tscsyncpolicy Legacy kann ich dir auf jedenfall empfehlen

pub fn set_bcd_store(default_settings: bool) {
  let command: &str;
  if default_settings {
      command = r#"
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope Process

[uint32]$useplatformclock = '0x260000a2'
[uint32]$useplatformtick = '0x260000a4'
[uint32]$disabledynamictick = '0x260000a5'
[uint32]$resumeobject = '0x23000003'

$BcdStore = (Invoke-CimMethod -ClassName BcdStore -Arguments @{File = ([System.String]::Empty)} -MethodName OpenStore -Namespace root\wmi).Store
$BcdBootMgrObject = (Invoke-CimMethod -Arguments @{Id = '{9dea862c-5cdd-4e70-acc1-f32b344d4795}'} -MethodName OpenObject -InputObject $BcdStore).Object
$BcdGuid = (Invoke-CimMethod -InputObject $BcdBootMgrObject -MethodName GetElement -Arguments @{Type = $resumeobject}).Element.Id
$BcdObject = (Invoke-CimMethod -Arguments @{Id = $BcdGuid} -MethodName OpenObject -InputObject $BcdStore).Object

try {
    Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformclock} -ErrorAction Stop | Out-Null
    Invoke-CimMethod -InputObject $BcdObject -MethodName DeleteElement -Arguments @{Type = $useplatformclock} | Out-Null
    Write-Host "delete setting: useplatformclock"
} catch {}

try {
    Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $disabledynamictick} -ErrorAction Stop | Out-Null
    Invoke-CimMethod -InputObject $BcdObject -MethodName DeleteElement -Arguments @{Type = $disabledynamictick} | Out-Null
    Write-Host "delete setting: disabledynamictick"
} catch {}

try {
    Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformtick} -ErrorAction Stop | Out-Null
    Invoke-CimMethod -InputObject $BcdObject -MethodName DeleteElement -Arguments @{Type = $useplatformtick} | Out-Null
    Write-Host "delete setting: useplatformtick"
} catch {}

"#;
  } else {
    command = r#"
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope Process

[uint32]$useplatformclock = '0x260000a2'
[uint32]$useplatformtick = '0x260000a4'
[uint32]$disabledynamictick = '0x260000a5'
[uint32]$resumeobject = '0x23000003'

$BcdStore = (Invoke-CimMethod -ClassName BcdStore -Arguments @{File = ([System.String]::Empty)} -MethodName OpenStore -Namespace root\wmi).Store
$BcdBootMgrObject = (Invoke-CimMethod -Arguments @{Id = '{9dea862c-5cdd-4e70-acc1-f32b344d4795}'} -MethodName OpenObject -InputObject $BcdStore).Object
$BcdGuid = (Invoke-CimMethod -InputObject $BcdBootMgrObject -MethodName GetElement -Arguments @{Type = $resumeobject}).Element.Id
$BcdObject = (Invoke-CimMethod -Arguments @{Id = $BcdGuid} -MethodName OpenObject -InputObject $BcdStore).Object

function set_element {
  param(
      [uint32]$key,
      [int]$value,
      [string]$keyname
  )
  $SetElementParams = @{
    MethodName = 'SetBooleanElement'
    InputObject = $BcdObject
    Arguments = @{
      Type = $key
      Boolean = $value
    }
  }
  Write-Host "write setting: $($keyname) = $(If($value -eq 1) {"Yes"} else {"No"})"
  Invoke-CimMethod @SetElementParams | Out-Null
}

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformclock} -ErrorAction Stop
  if ($result.Element.Boolean) {
    set_element $useplatformclock 0 "useplatformclock"
  }
} catch {
  set_element $useplatformclock 0 "useplatformclock"
}

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $disabledynamictick} -ErrorAction Stop
  if (!$result.Element.Boolean) {
    set_element $disabledynamictick 1 "disabledynamictick"
  }
} catch {
  set_element $disabledynamictick 1 "disabledynamictick"
}

try {
  $result = Invoke-CimMethod -InputObject $BcdObject -MethodName GetElement -Arguments @{Type = $useplatformtick} -ErrorAction Stop
  if (!$result.Element.Boolean) {
    set_element $useplatformtick 1 "useplatformtick"
  }
} catch {
  set_element $useplatformtick 1 "useplatformtick"
}

"#;
  }
    match powershell_script::run(command, false) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("\x1b[0;93m{}\x1b[0m ", e);
        }
    }
}
