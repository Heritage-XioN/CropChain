import app from "./app";
import { getOrCreateWallet } from './services/wallet.service'

;(async () => {
  const wallet = await getOrCreateWallet('test-user-id')
  console.log(wallet.publicKey)
})()

const PORT = process.env.PORT || 4000;

app.listen(PORT, () => {
  console.log(`🚀 CropChain backend running on port ${PORT}`);
});

