FROM scratch

COPY ./musl_release /regal
COPY web /web

CMD ["/regal", "-c", "/config/config.json", "-C", "/cache"]