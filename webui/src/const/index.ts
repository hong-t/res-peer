export enum Apps {
  feedApp = '986ae618dc4e82d2eca000670201e561cec00022bfbe9d809894f93dc6963a6dfdd8e1b11cee99719a62efe805173350e9d3d060749dd979670018e716ef40665a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15050000000000000000000000',
  creditApp = '1b0b7df1a5a2a54095d0d918df8af4a8d93f2fcac61cc20bb2fbdacb938c432d9968b607430e76a12793ba4c330a44dedc6250dfc0f073d73af71786df2b42ce5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15010000000000000000000000',
  marketApp = '3cd702e35aebf4fdf441a76d255a4764125048bfd1d62abe03a82161b318c3554a493ee7544f45bbbf0c7a942f5d6869983b55052ac29acceaaa43c5701bd7ad5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15070000000000000000000000',
  reviewApp = '802a23c1e2ec1be71f52d0c995a4d089070496bd774321ca086bf769d2f8ddca4ec0671b4d81f208d5ed02ae88b1110869db3a910fbc8dcad2264e3aaea444915a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15090000000000000000000000',
  activityApp = 'd0a66216faa1e2f1952cfd3b9f57d1a6f2d5777d7ed9fbdef54d13b4008cf8d44e833391cf8a032cd2d5c75d10e1d1ede1d0a1836b1d8a69c42405083b344bc15a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb150b0000000000000000000000',
  foundationApp = 'ccc878a445db7151d2845316dbfb3fe0d8d86688e0589b7d06b4a40bdb2fb43926b9a9091bd5f268d7762471eaec44e64af865516acd16534c17bd8bb7485f7f5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15030000000000000000000000',
  blobGatewayApp = '4865b24b045e87aa5ff397c2d289ab55c50800b9a236ddfcaa41a281b0cd92633b93ef88ae9e8661578700fec02df7b3380cdddfae2a258d197a7a0017a9100a5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb150d0000000000000000000000',
  cpRegistryApp = 'f5731c51cc672c340069d76c961ec282a99aba0ad85367f0899e53502480ae432d851f2da38725112cca7368021b14d07d74ae1705b2134641492f65b97119715a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb150f0000000000000000000000',
  copilotCpuApp = '1194a014ef57d8528320e497ae59a2563d96d09017de984b4b06d50852d8f5607b625ae5c8b7d2eb68114d282d573f09113a80703f557bc0939a9d6636ce0ae65a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15110000000000000000000000',
  copilotGpuApp = '1194a014ef57d8528320e497ae59a2563d96d09017de984b4b06d50852d8f5607b625ae5c8b7d2eb68114d282d573f09113a80703f557bc0939a9d6636ce0ae65a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15130000000000000000000000',
  illustratorCpuApp = '220c7bfd68a0c255c13fce80e5ebaca8f5e0384cd78de525c3fe21f5e0e984762290c26b56db79f5132cebe03c86e58c218855787f6cbe1cf57392832005022b5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15150000000000000000000000',
  illustratorGpuApp = '220c7bfd68a0c255c13fce80e5ebaca8f5e0384cd78de525c3fe21f5e0e984762290c26b56db79f5132cebe03c86e58c218855787f6cbe1cf57392832005022b5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15170000000000000000000000'
}

export const appIds = Object.values(Apps)

/// Chain which is the application originally deployed
export const appDeployChain = '5a6653f843f67294525ed8d7ac0d1c8b1e872a6b77e801dcbf7a3bd376fddb15'
export const appDeployOwner = 'd48a1daf5bbc537fc7c585013b059127119a824136b397eefe3955fb7b4e4a13'

/// Port should be set with different service
export const port = '9081'
export const host = 'localhost'
