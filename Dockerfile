ARG BASEIMG="debian:stretch"
ARG BUILDIMG="rust:slim-stretch"
FROM $BUILDIMG as builder

ENV BUILDDIR "/usr/src/app"

WORKDIR ${BUILDDIR} 

COPY . ${BUILDDIR}/

RUN cargo build --release

FROM $BASEIMG
LABEL maintainer="Nate Catelli <ncatelli@packetfire.org>"
LABEL description="Container for mud"

ENV SERVICE_USER "mud"
ENV APP_NAME="mud"
ENV BUILDDIR "/usr/src/app"

RUN groupadd ${SERVICE_USER} && \
    useradd -m --gid ${SERVICE_USER} ${SERVICE_USER}

COPY --from=builder ${BUILDDIR}/target/release/${APP_NAME} /opt/${APP_NAME}/bin/

RUN chown -R ${SERVICE_USER}:${SERVICE_USER} /opt/${APP_NAME} && \
    chmod +x /opt/${APP_NAME}/bin/${APP_NAME}

WORKDIR "/opt/$APP_NAME/"
USER ${SERVICE_USER}

ENTRYPOINT [ "./bin/mud" ]
CMD [ "-h" ]
