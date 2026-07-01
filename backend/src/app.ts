import express from 'express'
import routes from './routes'
import ussdRoutes from './routes/ussd'

const app = express()
app.use('/api/v1', ussdRoutes)

app.use(express.json())
app.use('/api/v1', routes)

export default app;