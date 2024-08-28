# 1. define base image untuk build
FROM rust:1.80.1-bullseye AS builder

# 2. define list dependency untuk build (define config list build) non-root
ENV USER=app
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .

# 3. build apps
RUN cargo build --release --locked
RUN strip -s target/release/b2rust

# 4. define image untuk deployment
FROM gcr.io/distroless/cc-debian12 AS runner
WORKDIR /app

# 5. define config untuk deployment (non-root)
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/b2rust ./

# 6. run app mode production untuk deployment
CMD ["./b2rust"]