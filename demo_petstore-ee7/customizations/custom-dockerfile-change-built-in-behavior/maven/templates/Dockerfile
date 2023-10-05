# Copyright IBM Corporation 2021
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# Use Red Hat Universal Base Image (UBI) as the base image
FROM registry.access.redhat.com/ubi8/ubi-minimal:latest

# Environment variables
ENV PORT 8080

# Update and install necessary packages
RUN microdnf update && \
    microdnf install -y java-1.8.0-openjdk-devel tar gzip shadow-utils && \
    microdnf clean all

# Set the WILDFLY_VERSION env variable
ENV WILDFLY_BASE_VERSION 26.0.0.Final
ENV WILDFLY_VERSION preview-$WILDFLY_BASE_VERSION
ENV WILDFLY_SHA1 d70903c335dd7a5678484c56a3dc45388c7ff9bd
ENV JBOSS_HOME /opt/jboss/wildfly

# Add WildFly distribution to /opt and set permissions
RUN cd $HOME && \
    curl -L -O https://github.com/wildfly/wildfly/releases/download/$WILDFLY_BASE_VERSION/wildfly-$WILDFLY_VERSION.tar.gz && \
    sha1sum wildfly-$WILDFLY_VERSION.tar.gz | grep $WILDFLY_SHA1 && \
    tar xf wildfly-$WILDFLY_VERSION.tar.gz && \
    mkdir -p $JBOSS_HOME && \
    mv $HOME/wildfly-$WILDFLY_VERSION/* $JBOSS_HOME && \
    rm wildfly-$WILDFLY_VERSION.tar.gz && \
    adduser -r jboss && \
    chown -R jboss:0 ${JBOSS_HOME} && \
    chmod -R g+rw ${JBOSS_HOME}

# Ensure signals are forwarded to the JVM process correctly for graceful shutdown
ENV LAUNCH_JBOSS_IN_BACKGROUND true

# Change to the jboss user
USER jboss

# Copy the application WAR file from the previous build stage
COPY --from=petstoreee7-buildstage /app/target/applicationPetstore.war ${JBOSS_HOME}/standalone/deployments/

# Expose the application port
EXPOSE 8080

# Set the default command to run on boot
# This will boot WildFly in standalone mode and bind to all interfaces
CMD ["/opt/jboss/wildfly/bin/standalone.sh", "-b", "0.0.0.0", "-bmanagement", "0.0.0.0"]
