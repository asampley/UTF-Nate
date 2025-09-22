{
  dockerTools,
  drv,
  entrypoint ? null,
  ...
}:
dockerTools.buildLayeredImage {
  name = drv.pname;
  tag = drv.version;
  contents = [ drv ] ++ drv.propagatedBuildInputs;
  config.Entrypoint = entrypoint;
}
// (if entrypoint != null then { config.Entrypoint = entrypoint; } else { })
