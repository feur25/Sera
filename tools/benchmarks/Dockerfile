FROM python:3.11-slim
ARG TOOL
ENV GLIBC_TUNABLES=glibc.rtld.optional_static_tls=4096
RUN pip install --no-cache-dir numpy==1.26.4 \
    && if [ "$TOOL" = "seraplot" ]; then pip install --no-cache-dir seraplot==2.8.8; fi \
    && if [ "$TOOL" = "matplotlib" ]; then pip install --no-cache-dir matplotlib==3.9.2; fi \
    && if [ "$TOOL" = "plotly" ]; then pip install --no-cache-dir plotly==5.24.1; fi
COPY bench_worker.py orchestrator.py warm_worker.py warm_orchestrator.py /bench/
WORKDIR /bench
