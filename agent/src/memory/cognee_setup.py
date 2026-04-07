import cognee
import asyncio
import os
from dotenv import load_dotenv

# Use full path to .env.elite if necessary
load_dotenv(".env.elite")

async def setup_memory():
    # Inicializa o Cognee com backend local
    try:
        # Cognee 0.5.7 API
        await cognee.prune.prune_system()
        # await cognee.initialize() # Not found in 0.5.7
        print("Cognee Memory Mesh Pruned.")
    except Exception as e:
        print(f"Error initializing Cognee: {e}")

if __name__ == "__main__":
    asyncio.run(setup_memory())
