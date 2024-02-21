FROM gitpod/workspace-full

RUN git clone https://github.com/hktprotocol/hktcore.git --depth 1 /home/gitpod/hktcore
RUN bash -cl "cd /home/gitpod/hktcore && cargo build && cargo test"
