# Minimal API image — not required for every dev task (TECH_STACK).
FROM python:3.14-slim
WORKDIR /app
COPY pyproject.toml ./
RUN pip install --no-cache-dir -e .
COPY python/ python/
COPY services/api/ services/api/
EXPOSE 8000
CMD ["uvicorn", "meta_quant_api.main:app", "--host", "0.0.0.0", "--port", "8000"]
