pub fn generate_launch_json_content() -> String {
  return format!(r#"
{{
  "configurations": [
    {{
      "name": "C/C++: g++.exe build and debug active file",
      "type": "cppdbg",
      "request": "launch",
      "program": "${{fileDirname}}\\bin\\${{fileBasenameNoExtension}}.exe",
      "args": [],
      "stopAtEntry": false,
      "cwd": "${{fileDirname}}",
      "environment": [],
      "externalConsole": true,
      "MIMode": "gdb",
      "miDebuggerPath": "gdb",
      "setupCommands": [
        {{
          "description": "Enable pretty-printing for gdb",
          "text": "-enable-pretty-printing",
          "ignoreFailures": true
        }},
        {{
          "description": "Set Disassembly Flavor to Intel",
          "text": "-gdb-set disassembly-flavor intel",
          "ignoreFailures": true
        }}
      ],
      "preLaunchTask": "C/C++: g++.exe build active file"
    }}
  ],
"version": "2.0.0"
}}
"#)
}