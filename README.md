# zig-project-backend

This is the backend for a university project aimed at helping people quit smoking.

---
## How to setup

### 1. Running via Docker (recommended)

#### Requirements
- Docker
- Docker Compose

#### Steps

1. **Get the source code**:
   - **Option 1: Clone the repository (recommended)**  
     ```bash
     git clone https://github.com/DaTiMy/zig-project-backend.git
     cd zig-project-backend-main
     ```

   - **Option 2: Download ZIP from GitHub**  
     - Click `Code` > `Download ZIP`
     - Extract the ZIP and open it in your terminal or editor

2. **Start the stack**:
   ```bash
   docker compose up --build
   ```

3. The backend should now be accessible at `http://0.0.0.0:8000`.

---

### 2. Running via Cargo (manually)

#### Requirements
- Rust toolchain
- MySQL/MariaDB server

#### Steps

1. **Get the source code**:
   - **Option 1: Clone the repository**
     ```bash
     git clone https://github.com/DaTiMy/zig-project-backend.git
     cd zig-project-backend-main
     ```
   - **Option 2: Download ZIP**
     - Click `Code` > `Download ZIP`
     - Extract the ZIP and open it in your terminal or editor

2. **Create a `.env` file**:
    ```env
    HTTP_HOST=0.0.0.0
    HTTP_PORT=8000
    DATABASE_URL=mysql://username:password@host:port/database
    ```
    
    > `HTTP_HOST` defaults to `0.0.0.0` to allow access from all network interfaces (e.g., Arduino, browser, etc.).  
    > `HTTP_PORT` defaults to `8000` as a common development port.  
    > `DATABASE_URL` must be provided — the application will not start without it.

3. **Build and run**:
   ```bash
   cargo build --release
   cargo run --release
   ```

4. The backend will listen on the host/port you configured.

---

## API Endpoints

- `GET /health` → Health check
- `POST /zigs` → Create a new Zig entry
- `GET /zigs/{id}` → Get Zig data by ID
- `POST /zigs/{id}/button-increment` → Increment button counter
- `POST /zigs/{id}/ash-increment` → Increment ash counter

---

## ❗ Notes

- When running on a local network, use `0.0.0.0` as `HTTP_HOST` so external clients (e.g., Arduino) can reach the server.
- Ensure the `DATABASE_URL` is correctly set for your MySQL/MariaDB instance.
- Make sure port `8000` is open and not blocked by any firewall.

