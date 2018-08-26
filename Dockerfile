FROM bitnami/minideb:stretch
COPY target/release/oto-tiny-http /app
CMD /app
