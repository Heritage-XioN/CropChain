import crypto from 'crypto'

const ALGORITHM = 'aes-256-gcm'
const KEY = crypto
  .createHash('sha256')
  .update(process.env.WALLET_ENCRYPTION_KEY || 'dev-key')
  .digest()

export function encrypt(text: string) {
  const iv = crypto.randomBytes(16)
  const cipher = crypto.createCipheriv(ALGORITHM, KEY, iv)

  let encrypted = cipher.update(text, 'utf8', 'hex')
  encrypted += cipher.final('hex')

  const tag = cipher.getAuthTag().toString('hex')

  return `${iv.toString('hex')}:${tag}:${encrypted}`
}

export function decrypt(payload: string) {
  const [ivHex, tagHex, encrypted] = payload.split(':')

  const decipher = crypto.createDecipheriv(
    ALGORITHM,
    KEY,
    Buffer.from(ivHex, 'hex')
  )

  decipher.setAuthTag(Buffer.from(tagHex, 'hex'))

  let decrypted = decipher.update(encrypted, 'hex', 'utf8')
  decrypted += decipher.final('utf8')

  return decrypted
}