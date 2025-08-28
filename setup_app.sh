#!/bin/bash

# Simple script to set up and run the CPU Usage app

echo "Setting up CPU Usage app..."

# Make sure the binary is executable
chmod +x CPUUsage.app/Contents/MacOS/cpusage

# Create a simple launcher script for easy access
cat > launch_cpusage.sh << 'EOF'
#!/bin/bash
cd "$(dirname "$0")"
open CPUUsage.app
EOF

chmod +x launch_cpusage.sh

echo "✅ CPU Usage app is ready!"
echo ""
echo "To run the app:"
echo "1. Double-click on CPUUsage.app in Finder"
echo "2. Or run: ./launch_cpusage.sh"
echo "3. Or run: open CPUUsage.app"
echo ""
echo "The app will appear in your menu bar with a ⚡ icon showing CPU percentage."
echo "Right-click the menu bar item to see CPU details and quit option."
