export const env = {
  PORT: process.env.PORT || 4000,
  NODE_ENV: process.env.NODE_ENV || 'development',
  WALLET_ENCRYPTION_KEY: process.env.WALLET_ENCRYPTION_KEY || 'super-secret-dev-key'
}