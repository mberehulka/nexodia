import bpy, struct, os, time
from io import BufferedWriter
from pathlib import Path

start = time.time()

os.makedirs("./assets", exist_ok=True)

def write_u32(f: BufferedWriter, v: any):
    f.write(v.to_bytes(4, byteorder='big', signed=False))
def write_u8(f: BufferedWriter, v: any):
    if v > 255: raise Exception("Value is bigger than 255")
    f.write(v.to_bytes(1, byteorder='big', signed=False))
def write_str(f: BufferedWriter, v: any):
    f.write(str.encode(str(v))+b'#')
def write_mat4x4(f: BufferedWriter, mat: any):
    f.write(struct.pack(">f", mat[0][0])); f.write(struct.pack(">f", mat[1][0]))
    f.write(struct.pack(">f", mat[2][0])); f.write(struct.pack(">f", mat[3][0]))
    f.write(struct.pack(">f", mat[0][1])); f.write(struct.pack(">f", mat[1][1]))
    f.write(struct.pack(">f", mat[2][1])); f.write(struct.pack(">f", mat[3][1]))
    f.write(struct.pack(">f", mat[0][2])); f.write(struct.pack(">f", mat[1][2]))
    f.write(struct.pack(">f", mat[2][2])); f.write(struct.pack(">f", mat[3][2]))
    f.write(struct.pack(">f", mat[0][3])); f.write(struct.pack(">f", mat[1][3]))
    f.write(struct.pack(">f", mat[2][3])); f.write(struct.pack(">f", mat[3][3]))

def get_armature():
    for object in bpy.data.objects:
        if object.type == "ARMATURE":
            object.select_set(True)
            return object
    raise Exception("No Armature found")

def clear_scene():
    bpy.ops.wm.read_factory_settings(use_empty=True)

def initialize_file(path: Path, folder: str, type: bytes):
    name = path.name.split('.')[0]
    root = Path(f"./.compiled/{folder}/").joinpath(path.parent.relative_to(f"assets/{folder}/"))
    os.makedirs(root, exist_ok=True)
    f = open(root.joinpath(name+".low"), "wb+")
    f.write(type)
    return f

def export_bones(f: BufferedWriter):
    bones_length = len(bpy.context.selected_pose_bones)
    if bones_length >= 255: raise Exception("Armature must have less than 255 bones")
    write_u8(f, bones_length)

def set_last_frame():
    if bpy.data.actions:
        action_list = [action.frame_range for action in bpy.data.actions]
        keys = (sorted(set([item for sublist in action_list for item in sublist])))
        bpy.context.scene.frame_end = int(keys[-1])
    else: raise Exception("No actions found")

def export_frames(f: BufferedWriter):
    frames = bpy.context.scene.frame_end
    write_u32(f, frames)
    for bone in bpy.context.selected_pose_bones:
        write_str(f, bone.name)
        for frame in range(frames):
            bpy.context.scene.frame_set(frame)
            bpy.context.view_layer.update()
            write_mat4x4(f, bone.matrix @ (bone.parent.matrix.inverted_safe()) if bone.parent else bone.matrix )

def export_animation(path: Path):
    f = initialize_file(path, "animations", b"A")

    bpy.ops.object.mode_set(mode='POSE')

    export_bones(f)
    set_last_frame()
    export_frames(f)

    f.close()

for path in Path("assets").glob("**/*.fbx"):
    clear_scene()
    print("\nImporting fbx animation: "+str(path))
    bpy.ops.import_scene.fbx(filepath=str(path))
    export_animation(path)

print(f"Animations compiled in: {(time.time() - start):.1f} sec")