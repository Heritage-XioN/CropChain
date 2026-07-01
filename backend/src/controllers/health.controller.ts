import { Request, Response } from 'express'
import { getHealthStatus } from '../services/health.service'

export const healthCheck = (req: Request, res: Response) => {
  res.json(getHealthStatus())
}