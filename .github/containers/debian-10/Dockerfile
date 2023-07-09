LABEL org.opencontainers.image.source https://github.com/STBoyden/codectrl
FROM debian:10

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
