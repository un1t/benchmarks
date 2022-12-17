const os = require("os");
const cluster = require("cluster");
const fastify = require('fastify')({ logger: true, disableRequestLogging: true })
const pg = require('pg')
var parse = require('pg-connection-string').parse;
require('dotenv').config()

const dbConfig = parse(process.env.DATABASE_URL)

const pgPool = new pg.Pool({
    host: dbConfig.host,
    user: dbConfig.user,
    database: dbConfig.database,
    max: 1,
})

const clusterWorkerSize = os.cpus().length;

fastify.get('/', async (request, reply) => {
    const { rows } = await pgPool.query('SELECT id, title, content from words limit 100')
    return rows
})

fastify.get('/ping', async (request, reply) => {
    return "OK"
})

const start = async () => {
    try {
        await fastify.listen({ port: 3000 })
    } catch (err) {
        fastify.log.error(err)
        process.exit(1)
    }
}

if (clusterWorkerSize > 1) {
    if (cluster.isMaster) {
        for (let i = 0; i < clusterWorkerSize; i++) {
            cluster.fork();
        }

        cluster.on("exit", function (worker) {
            console.log("Worker", worker.id, " has exited.")
        })
    } else {
        start();
    }
} else {
    start();
}