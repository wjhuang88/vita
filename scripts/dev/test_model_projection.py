import unittest
from run_model_vm import project_config

CONFIG = '''provider = "test"
model = "test-model"
[providers.test]
protocol = "openai-chat"
base_url = "https://example.invalid/v1"
api_key = "INLINE_CANARY"
api_key_env = "TEST_KEY"
[hooks]
secret = "UNRELATED_CANARY"
'''
class ProjectionTests(unittest.TestCase):
    def test_projection_and_env_precedence(self):
        result = project_config(CONFIG, {"TEST_KEY": "ENV_CANARY"})
        self.assertEqual(result["api_key"], "ENV_CANARY")
        self.assertNotIn("UNRELATED_CANARY", str(result))
        self.assertNotIn("INLINE_CANARY", str(result))
        self.assertEqual(set(result), {"protocol", "model", "base_url", "api_key"})
    def test_rejects_unsafe_endpoint_and_protocol(self):
        for source in [CONFIG.replace("https://", "http://"), CONFIG.replace("https://", "https://secret@"), CONFIG.replace("openai-chat", "unknown")]:
            with self.assertRaises(ValueError): project_config(source, {})

if __name__ == "__main__": unittest.main()
