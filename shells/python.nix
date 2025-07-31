{ pkgs }:

let
  version = "3.13";

  concatMajorMinor = v:
    pkgs.lib.pipe v [
      pkgs.lib.versions.splitVersion
      (pkgs.lib.sublist 0 2)
      pkgs.lib.concatStrings
    ];

  python = pkgs."python${concatMajorMinor version}";
in
pkgs.mkShellNoCC {
  venvDir = ".venv";

  postShellHook = ''
    venvVersionWarn() {
      local venvVersion
      venvVersion="$("$venvDir/bin/python" -c 'import platform; print(platform.python_version())')"

      [[ "$venvVersion" == "${python.version}" ]] && return

      cat <<EOF
Warning: Python version mismatch: [$venvVersion (venv)] != [${python.version}]
         Delete '$venvDir' and reload to rebuild for version ${python.version}
EOF
    }

    venvVersionWarn
  '';

  packages = [
    python.pkgs.venvShellHook
    python.pkgs.pip
    python.pkgs.pandas
    python.pkgs.numpy
    python.pkgs.matplotlib
    python.pkgs.seaborn
    python.pkgs.plotly
    python.pkgs.jupyterlab
    python.pkgs.ipython
    python.pkgs.scipy
    python.pkgs.pyyaml
    pkgs.git
    pkgs.nodejs
  ];
}
