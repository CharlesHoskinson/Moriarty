> For the complete documentation index, see [llms.txt](/llms.txt)

# Set up Cardano-db-sync

This guide provides step-by-step instructions for setting up Cardano-db-sync for Midnight.

## Prerequisites[​](#prerequisites "Direct link to Prerequisites")

Before setting up Cardano-db-sync, ensure you have the following:

* [Cardano node set up](/nodes/cardano-node.md) with a running node.
* Sufficient resources (CPU, memory, and storage).

## Create PostgreSQL database[​](#create-postgresql-database "Direct link to Create PostgreSQL database")

`Cardano-db-sync` requires a `PostgreSQL` backend to index blockchain data into a relational schema.

### Install PostgreSQL[​](#install-postgresql "Direct link to Install PostgreSQL")

Run the following commands to install PostgreSQL:

```
sudo apt install curl ca-certificates -y

sudo install -d /usr/share/postgresql-common/pgdg

sudo curl -s -o /usr/share/postgresql-common/pgdg/apt.postgresql.org.asc --fail https://www.postgresql.org/media/keys/ACCC4CF8.asc

sudo sh -c 'echo "deb [signed-by=/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc] https://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'

sudo apt update && sudo apt -y install postgresql-17 postgresql-server-dev-17
```

### Configure roles and database[​](#configure-roles-and-database "Direct link to Configure roles and database")

Start the PostgreSQL shell:

```
sudo -i -u postgres psql
```

Create the user `midnight` and ensure the `midnight` user has a known password + full privileges:

```
CREATE USER midnight WITH PASSWORD 'your_actual_password';

ALTER ROLE midnight WITH SUPERUSER CREATEDB;

CREATE DATABASE cexplorer;
```

These commands securely log into PostgreSQL as admin, create the user `midnight` and ensure the `midnight` user has a known password + full privileges, and create the empty `cexplorer` database.

### Configure authentication[​](#configure-authentication "Direct link to Configure authentication")

Create a `.pgpass` file to allow Cardano-db-sync to connect without manual password entry:

```
# Set variable for postgres password

export POSTGRES_PASSWORD='your_actual_password'



# Set the variable to a hidden file in your home directory

export PGPASSFILE="${HOME}/.pgpass"
```

important

Ensure you replace `your_actual_password` with your actual secure password.

Write the connection string to the `.pgpass` file:

```
echo "/var/run/postgresql:5432:cexplorer:midnight:$POSTGRES_PASSWORD" > "$PGPASSFILE"
```

Set strict permissions for the `.pgpass` file. PostgreSQL ignores the file if it's world-readable:

```
chmod 0600 "$PGPASSFILE"
```

Run a connection test:

```
psql -h /var/run/postgresql -U midnight -d cexplorer -c "SELECT current_user; SELECT now();"
```

How to interpret the results:

* **Success**: If successful, then it returns a table showing `midnight` and the current timestamp without asking for a password.
* **Password Prompt**: If it asks for a password, then the `.pgpass` file isn't being read correctly. Likely a mismatch in the host, port, or database name.
* **Access Denied**: If you get `database "cexplorer" does not exist`, then you need to create the database first.

Verify the `.pgpass` file exists including contents:

```
cat ~/.pgpass
```

Example output:

```
/var/run/postgresql:5432:cexplorer:midnight:YOUR_PASSWORD
```

The `~/.pgpass` file is a configuration file used by PostgreSQL client tools such as `psql` and `pg_dump` to automatically retrieve passwords without requiring manual entry or hard-coding them in scripts.

### PostgreSQL 17 configuration tuning[​](#postgresql-17-configuration-tuning "Direct link to PostgreSQL 17 configuration tuning")

important

PostgreSQL tuning is **required** when targeting Mainnet. Without it, Cardano-db-sync synchronization will take an extremely long time to complete.

Modify your `/etc/postgresql/17/main/postgresql.conf` file:

Find and update these specific lines. Remove the `#` to uncomment them:

| Configuration                          | Description                                                                                                            |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `shared_buffers = 16GB`                | This allows PostgreSQL to keep much more of the Cardano ledger in active memory.                                       |
| `maintenance_work_mem = 4GB`           | This is the most important for your current "Building Index" phase. It gives the index sorter more elbow room to work. |
| `max_parallel_maintenance_workers = 4` | This allows 4 CPU cores to collaborate on building that single index.                                                  |
| `effective_cache_size = 48GB`          | This helps the PostgreSQL planner understand how much total RAM it can rely on for caching.                            |

## Setup Cardano-db-sync[​](#setup-cardano-db-sync "Direct link to Setup Cardano-db-sync")

This section covers downloading and installing Cardano-db-sync binaries and config files.

important

Use Cardano-db-sync [13.7.1.0](https://github.com/IntersectMBO/cardano-db-sync/releases/tag/13.7.1.0) together with Cardano-node [11.0.1](https://github.com/IntersectMBO/cardano-node/releases/tag/11.0.1). Because of the Van Rossem upgrade changes, the Cardano-db-sync database must be replayed against Cardano-node 11.0.1. Upgrading to this Cardano-db-sync version is not just a binary swap. The existing database needs to be replayed before it is usable.

### Download Cardano-db-sync binaries[​](#download-cardano-db-sync-binaries "Direct link to Download Cardano-db-sync binaries")

Set the network to either, "preview", "mainnet", or "preprod":

```
NETWORK="preprod"
```

Download the Cardano-db-sync binaries and extract them into a temporary directory:

```
mkdir -p ~/tmp

cd ~/tmp

curl -L -O https://github.com/IntersectMBO/cardano-db-sync/releases/download/13.7.1.0/cardano-db-sync-13.7.1.0-linux.tar.gz

tar -xzf cardano-db-sync-13.7.1.0-linux.tar.gz
```

Move Cardano-db-sync binaries to the local bin directory:

```
mkdir -p ~/.local/bin

cp bin/* ~/.local/bin/

chmod +x ~/.local/bin/cardano-db-sync*
```

Verify Cardano-db-sync installation:

```
cardano-db-sync --version

which cardano-db-sync
```

Move the downloaded `db-sync` schema into the `~/cardano-data` directory:

```
mkdir -p ~/cardano-data/

sudo mv ~/tmp/schema ~/cardano-data/
```

Download the Cardano-db-sync `config.json` file:

```
cd ~/cardano-data

curl -O https://book.world.dev.cardano.org/environments/$NETWORK/db-sync-config.json
```

Update the Cardano-db-sync `config.json` file with the Cardano node configuration file:

```
sed -i "s|\"NodeConfigFile\": \"config.json\"|\"NodeConfigFile\": \"/home/midnight/.local/share/$NETWORK/config.json\"|" ~/cardano-data/db-sync-config.json
```

Verify Cardano-db-sync installation:

```
cardano-db-sync --version

which cardano-db-sync
```

<!-- -->

### Run Cardano-db-sync[​](#run-cardano-db-sync "Direct link to Run Cardano-db-sync")

important

Make sure your [Cardano node](/nodes/cardano-node.md) is running. Cardano-db-sync uses the Cardano database (provided by Cardano-node) to index chain.

Set the environment variable for the PostgreSQL password:

```
export PGPASSFILE="${HOME}/.pgpass"
```

Start Cardano-db-sync interactively in shell:

```
cardano-db-sync \

    --config /home/midnight/cardano-data/db-sync-config.json \

    --socket-path /home/midnight/cardano-data/db/node.socket \

    --schema-dir /home/midnight/cardano-data/schema \

    --state-dir /home/midnight/cardano-data/db-sync-state
```

info

Cardano-db-sync might delay for 5-20 minutes while it initializes. This is normal and expected.

Check the latest block height of the `cardano-db-sync`:

```
psql -d cexplorer -c "SELECT block_no, slot_no, time FROM block ORDER BY id DESC LIMIT 1;"
```

Get `cardano-db-sync` sync percentage:

```
psql -d cexplorer -c "

SELECT 

    100 * (EXTRACT(epoch FROM (MAX(time) AT TIME ZONE 'UTC')) - EXTRACT(epoch FROM (MIN(time) AT TIME ZONE 'UTC'))) 

    / (EXTRACT(epoch FROM (NOW() AT TIME ZONE 'UTC')) - EXTRACT(epoch FROM (MIN(time) AT TIME ZONE 'UTC'))) 

AS sync_percent 

FROM block;" 
```

Example output:

```
sync_percent

------------

45.00

(1 row)
```

This means that the Cardano-db-sync is 45% synced. You need to wait for the sync percentage to reach at least 99% before running Cardano-db-sync.

info

Get more interesting SQL queries from the Cardano-db-sync repository: <https://github.com/IntersectMBO/cardano-db-sync/blob/master/doc/interesting-queries.md>

### Setup Cardano-db-sync `systemd` service[​](#setup-cardano-db-sync-systemd-service "Direct link to setup-cardano-db-sync-systemd-service")

Create the service file for the Cardano-db-sync:

```
sudo vim /etc/systemd/system/cardano-db-sync.service
```

Paste the following contents into the service file:

```
[Unit]

Description=Cardano DB Sync

# Ensures db-sync doesn't start until the node is ready

After=cardano-node.service

Requires=cardano-node.service



[Service]

User=midnight

Type=simple

# Crucial: Tells the tool where to find the Postgres password

Environment="PGPASSFILE=/home/midnight/.pgpass"

WorkingDirectory=/home/midnight/cardano-data

ExecStart=/home/midnight/.local/bin/cardano-db-sync \

    --config /home/midnight/cardano-data/db-sync-config.json \

    --socket-path /home/midnight/cardano-data/db/node.socket \

    --schema-dir /home/midnight/cardano-data/schema \

    --state-dir /home/midnight/cardano-data/db-sync-state

# SIGINT is the "clean" way to shut down to prevent database corruption

KillSignal=SIGINT

Restart=always

RestartSec=10

# Increases the limit for open files (important for high-throughput DBs)

LimitNOFILE=32768



[Install]

WantedBy=multi-user.target
```

Enable and start the Cardano-db-sync service:

```
# Reload systemd to recognize new files

sudo systemctl daemon-reload



# Enable services to start on boot

sudo systemctl enable cardano-db-sync



# Start the services

sudo systemctl start cardano-db-sync 
```

View the status of the Cardano-db-sync service:

```
sudo systemctl status cardano-db-sync
```

View live logs:

```
# Follow Cardano-db-sync logs

journalctl -u cardano-db-sync -f
```

## Next steps[​](#next-steps "Direct link to Next steps")

With Cardano-db-sync fully synchronized, you can start your Midnight node. To learn more, see the [Set up full node](/nodes/full-node.md) guide.
