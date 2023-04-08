# Buss2

### What
A web app and backend for tracking bus routes and times in Agder, Norway.

### Why
The official AKT app is often slow, and loses track of favorite routes from time to time. Also, I needed an excuse to learn more Rust (specifically Axum and Diesel), and noticed bus data was available in a public API from Entur.

### How
The backend is written in Rust, using the Axum web framework, and Diesel ORM (with Postgres). The frontend is written in Vue 3, using Quasar.

### Deployment
#### Build locally
```
cd frontend
quasar build

cd ..

cd backend
cargo build --release
```

#### Upload files
Set `DEPLOY_TO` in `.env` and run `./deploy.sh`. This will upload the frontend and backend to the server. Be sure to build frontend and backend locally first.
#### Postgres
Create a database: 
```
sudo -u postgres psql
CREATE DATABASE buss2;
CREATE USER buss2 WITH ENCRYPTED PASSWORD 'secret';
GRANT ALL PRIVILEGES ON DATABASE buss2 TO buss2;
```

#### Caddy / systemd
This is set up to run with Caddy as a reverse proxy, and systemd to manage the service. 

If this is the first project:
```
mkdir /etc/caddy/sites-enabled
echo "import sites-enabled/*.caddy" >> /etc/caddy/Caddyfile
```

Then:
```
sudo ln -s "$(pwd)/config/Caddyfile" /etc/caddy/sites-enabled/buss2.caddy
sudo ln -s "$(pwd)/config/buss2.socket" /etc/systemd/system/
sudo ln -s "$(pwd)/config/buss2.service" /etc/systemd/system/
sudo systemctl enable --now buss2.socket
sudo systemctl enable --now buss2.service
```
