import { Keypair } from '@solana/web3.js'
import { encrypt, decrypt } from '../utils/crypto'
import { prisma } from '../config/prisma'

export async function getOrCreateWallet(userId: string) {
  const existing = await prisma.custodialWallet.findUnique({
    where: { userId }
  })

  if (existing) {
    return {
      publicKey: existing.publicKey,
      secretKey: Uint8Array.from(
        JSON.parse(decrypt(existing.encryptedSk))
      )
    }
  }

  const keypair = Keypair.generate()

  await prisma.custodialWallet.create({
    data: {
      userId,
      publicKey: keypair.publicKey.toBase58(),
      encryptedSk: encrypt(JSON.stringify(Array.from(keypair.secretKey)))
    }
  })

  return {
    publicKey: keypair.publicKey.toBase58(),
    secretKey: keypair.secretKey
  }
}