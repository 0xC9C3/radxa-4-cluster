FROM ubuntu:25.04 AS uf2-builder

# build the uf2

COPY uf2/CMakeLists.txt uf2/pico_extras_import_optional.cmake uf2/pico_sdk_import.cmake /uf2/
COPY uf2/pico-sdk /uf2/pico-sdk
COPY uf2/pwm /uf2/pwm

RUN apt-get update && \
    apt-get install -y git cmake gcc-arm-none-eabi libnewlib-arm-none-eabi libstdc++-arm-none-eabi-newlib python3 python3-pip && \
    cd /uf2 && \
    ls -la && \
    mkdir -p build && \
    cd build && \
    export PICO_SDK_PATH=/uf2/pico-sdk && \
    cmake .. && \
    make -j4 && \
    mkdir -p /out/ && cp pwm/pwm_fan.uf2 /out/ && \
    rm -rf /uf2 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

FROM rust:bookworm AS app-builder

WORKDIR /app-build
COPY . .

RUN mkdir -p /app-build/uf2/build/pwm

# libudev is required for the build
RUN apt-get update && \
    apt-get install -y libudev-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=uf2-builder /out/pwm_fan.uf2 /app-build/uf2/build/pwm/pwm_fan.uf2

RUN cargo build --release && mkdir -p /out/ && cp target/release/fan-manager /out/ && rm -rf /app-build/

FROM ubuntu:25.04 AS release

COPY --from=app-builder /out/fan-manager /bin/

# install gpioset
RUN apt-get update && \
    apt-get install -y libgpiod3 && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/bin/fan-manager"]