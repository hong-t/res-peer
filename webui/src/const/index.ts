export enum Apps {
  feedApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030600000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03080000000000000000000000',
  creditApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030000000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03020000000000000000000000',
  marketApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030900000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030b0000000000000000000000',
  reviewApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030c00000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030e0000000000000000000000',
  activityApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030f00000000000000020000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03110000000000000000000000',
  foundationApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030300000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03050000000000000000000000',
  blobGatewayApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031200000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03140000000000000000000000',
  cpRegistryApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031500000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03170000000000000000000000',
  copilotCpuApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031800000000000000000000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031a0000000000000000000000',
  copilotGpuApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031b00000000000000020000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031d0000000000000000000000',
  illustratorCpuApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d031e00000000000000020000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03200000000000000000000000',
  illustratorGpuApp = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d032100000000000000020000001db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03230000000000000000000000'
}

export const appIds = Object.values(Apps)

/// Chain which is the application originally deployed
export const appDeployChain = '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03'
export const appDeployOwner = '85731b60f89fd0d16505b63ec9754f4e1754e5cd1b432ce628f471c32eaa9687'

/// Port should be set with different service
export const port = '9080'
export const host = 'localhost'
