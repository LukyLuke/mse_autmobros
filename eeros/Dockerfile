FROM ghcr.io/online-robotics/eeros-docker-bbblue:1.0.5

VOLUME /project

RUN echo "#!/bin/bash" > /build.sh \
  && echo "mkdir -p /project/build" >> /build.sh \
  && echo "cd /project/build" >> /build.sh \
  && echo "cmake .." >> /build.sh \
  && echo "make" >> /build.sh \
  && chmod 0777 /build.sh

CMD [ "/bin/bash" ]
#CMD [ "/build.sh" ]
