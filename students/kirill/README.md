Porting https://github.com/cppalliance/tools-public/blob/master/agents/nash.md

First, tried to add a hand-written eval loop, but seems that's not needed — discussed with Vinnie.

Now, need to build (vibing is fine here) the frontend app to visualize the dashboard, determine a set of extra tools for runvil to interact with it — then port the prompt so that a good game theory-capable model can interactively update the app's state same as current prompt allows models to update the markdown.
