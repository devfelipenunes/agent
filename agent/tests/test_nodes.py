import unittest
from unittest.mock import MagicMock, patch
import sys
import os
import json

# Add parent dir to path to import graph
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from src.core.graph import orchestrator_node, AgentState

class TestAetherBrain(unittest.TestCase):
    def setUp(self):
        self.state: AgentState = {
            "query": "Olá",
            "chat_history": [],
            "findings": [],
            "artifact": "",
            "brief": "",
            "review_feedback": "",
            "current_status": "",
            "user_intent": ""
        }

    @patch('src.core.graph.llm')
    @patch('src.core.graph.emit_event')
    def test_orchestrator_simple_chat(self, mock_emit, mock_llm):
        # Mock LLM to return a simple greeting
        mock_response = MagicMock()
        mock_response.content = "Olá, como posso ajudar?"
        mock_llm.stream.return_value = [mock_response]
        
        result = orchestrator_node(self.state)
        
        self.assertEqual(result["user_intent"], "CHAT")
        # Verify that at least message_start and token were emitted
        calls = [call[0][0] for call in mock_emit.call_args_list]
        self.assertIn("message_start", calls)
        self.assertIn("token", calls)

    @patch('src.core.graph.llm')
    @patch('src.core.graph.emit_event')
    def test_orchestrator_research_intent(self, mock_emit, mock_llm):
        # Mock LLM to trigger research
        mock_response = MagicMock()
        mock_response.content = "SQUAD_REQUIRED: RESEARCH. Vou pesquisar isso."
        mock_llm.stream.return_value = [mock_response]
        
        self.state["query"] = "Pesquise sobre Bitcoin"
        result = orchestrator_node(self.state)
        
        self.assertEqual(result["user_intent"], "RESEARCH")

if __name__ == "__main__":
    unittest.main()
