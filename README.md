# Package manager CI samples

This repository servers as a sample/reference for building a releasing an app via automated CI/CD pipelines to different artifact registries and packaging the application for package managers.

**Currently implemented releases:**
- Docker (GitHub container registry)
- APT (deployment to an APT repository on an on-prem server)

**Todo:**
- Snapcraft
- RPM
- GitHub releases
- ...

## About the application

The deployed application is written in Rust and called *Whatstheweather*. It is a CLI app which fetches the current weather forecast from the [OpenWeather API](https://openweathermap.org/api) for set parameters (City, number of days in the future, etc)
