import { Router } from 'express'
import { healthCheck } from '../controllers/health.controller'

const router = Router()

router.get('/health', healthCheck)
router.get("/health", (_req, res) => {
  res.status(200).json({
    status: "ok",
    uptime: process.uptime(),
    timestamp: new Date().toISOString(),
  });
});

export default router