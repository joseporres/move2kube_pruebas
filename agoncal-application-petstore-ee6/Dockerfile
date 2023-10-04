# Use an official Maven image as a parent image
FROM maven:3.6-jdk-8

# Set environment variables for GlassFish
ENV GLASSFISH_VERSION 3.1.2.2
ENV GLASSFISH_HOME /glassfish3
ENV PATH $PATH:$GLASSFISH_HOME/bin

# Download and install GlassFish 3.x
RUN curl -O https://download.oracle.com/glassfish/${GLASSFISH_VERSION}/release/glassfish-${GLASSFISH_VERSION}.zip && \
    unzip glassfish-${GLASSFISH_VERSION}.zip -d / && \
    rm glassfish-${GLASSFISH_VERSION}.zip

# Expose ports (change them according to your application needs)
EXPOSE 8080 4848

# Copy the entire application source code to the container
COPY . /app

# Build the application and deploy it to GlassFish
WORKDIR /app
RUN mvn clean package && \
    cp target/applicationPetstore.war $GLASSFISH_HOME/glassfish/domains/domain1/autodeploy/

# Start GlassFish server
CMD ["/glassfish3/bin/asadmin", "start-domain", "--verbose"]