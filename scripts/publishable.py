import json
import subprocess
import re

metadata = subprocess.run(
    ["cargo", "metadata", "--format-version=1"],
    capture_output=True,
    text=True,
    check=True,
)
metadata_json = json.loads(metadata.stdout)
members = metadata_json["workspace_members"]
packages = {pkg["id"]: pkg for pkg in metadata_json["packages"]}
publishable = []
for member in members:
    if packages[member]["publish"] == []:  # False
        continue
    member = re.sub(r"^.*/", "", member)  # Remove leading path
    member = re.sub(r"^.*#(.+)@", "\\1#", member)  # Remove leading path
    member = re.sub(r"#.*$", "", member)  # Remove trailing version
    publishable.append(member)

# Move fontspector to end
if "fontspector" in publishable:
    publishable.remove("fontspector")
    publishable.append("fontspector")
print(" ".join(publishable))
