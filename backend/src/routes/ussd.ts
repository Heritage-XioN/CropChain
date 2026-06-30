import { Router } from 'express'

const router = Router()

router.post('/ussd', async (req, res) => {
  const { text, phoneNumber } = req.body

  if (!text) {
    return res.send('CON Welcome to CropChain\n1. Register')
  }

  res.send('END Feature coming soon')
})

export default router