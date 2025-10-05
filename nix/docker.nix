{
  dockerTools,
  drv,
  lib,
  ...
}:
dockerTools.buildLayeredImage {
  name = drv.pname;
  tag = drv.version;
  contents = [ drv ] ++ drv.propagatedBuildInputs;
  config = {
    entrypoint = if drv.meta ? mainProgram then [ (lib.getExe' drv drv.meta.mainProgram) ] else null;
  };
}
