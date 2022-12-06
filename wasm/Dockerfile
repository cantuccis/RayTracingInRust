FROM python:3.8-slim-buster

COPY index.html index.js worker.js raytrace_parallel.js raytrace_parallel_bg.wasm server.py ./

CMD [ "python3", "server.py"]
EXPOSE 80
