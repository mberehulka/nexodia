import bpy, struct, os, time, zstd, json
from io import BufferedWriter
from pathlib import Path
from mathutils import Quaternion, Vector

script_start = time.time()

os.makedirs("./assets", exist_ok=True)

class Settings:
    compression_level = 12
    animations = False
    def __init__(self, p: Path):
        if p.exists():
            data = json.load(open(p))
            if isinstance(data.get('compression_level'), int):
                self.compression_level = data["compression_level"]
            if isinstance(data.get('animations'), bool):
                self.animations = data["animations"]

def write_u32(b: bytearray, v: any):
    b.extend(v.to_bytes(4, byteorder='big', signed=False))
def write_u8(b: bytearray, v: any):
    if v > 255: raise Exception("Value is bigger than 255")
    b.extend(v.to_bytes(1, byteorder='big', signed=False))
def write_str(b: bytearray, v: any):
    b.extend(str.encode(str(v))+b'#')
def write_vec3(b: bytearray, v: any):
    b.extend(struct.pack(">f", v.x)); b.extend(struct.pack(">f", v.y)); b.extend(struct.pack(">f", v.z))
def write_vec4(b: bytearray, v: any):
    b.extend(struct.pack(">f", v.x)); b.extend(struct.pack(">f", v.y)); b.extend(struct.pack(">f", v.z)); b.extend(struct.pack(">f", v.w))
def write_mat4x4(b: bytearray, mat: any):
    b.extend(struct.pack(">f", mat[0][0])); b.extend(struct.pack(">f", mat[1][0])); b.extend(struct.pack(">f", mat[2][0])); b.extend(struct.pack(">f", mat[3][0]))
    b.extend(struct.pack(">f", mat[0][1])); b.extend(struct.pack(">f", mat[1][1])); b.extend(struct.pack(">f", mat[2][1])); b.extend(struct.pack(">f", mat[3][1]))
    b.extend(struct.pack(">f", mat[0][2])); b.extend(struct.pack(">f", mat[1][2])); b.extend(struct.pack(">f", mat[2][2])); b.extend(struct.pack(">f", mat[3][2]))
    b.extend(struct.pack(">f", mat[0][3])); b.extend(struct.pack(">f", mat[1][3])); b.extend(struct.pack(">f", mat[2][3])); b.extend(struct.pack(">f", mat[3][3]))
def write_mat4x4_decomposed(b: bytearray, mat: any):
    translation, rotation, scale = mat.decompose()
    write_vec3(b, translation)
    write_vec4(b, rotation)
    write_vec3(b, scale)

def get_armature():
    for object in bpy.data.objects:
        if object.type == "ARMATURE":
            object.select_set(True)
            return object
    raise Exception("No Armature found")

def clear_scenes():
    for scene in bpy.data.scenes:
        for ob in scene.objects:
            bpy.data.objects.remove(ob, do_unlink=True)

def set_last_frame():
    if bpy.data.actions:
        action_list = [action.frame_range for action in bpy.data.actions]
        keys = (sorted(set([item for sublist in action_list for item in sublist])))
        bpy.context.scene.frame_end = int(keys[-1])
    else: raise Exception("No actions found")

def export_frames(b: BufferedWriter):
    write_u8(b, len(bpy.context.selected_pose_bones)) # Bones count
    frames = bpy.context.scene.frame_end
    write_u32(b, frames) # Frames count
    for frame in range(frames):
        bpy.context.scene.frame_set(frame)
        bpy.context.view_layer.update()

        write_mat4x4_decomposed(b, bpy.context.object.matrix_local)

        for bone in bpy.context.selected_pose_bones:
            write_mat4x4_decomposed(b, bone.matrix @ bone.parent.matrix.inverted() if bone.parent else bone.matrix)
            write_mat4x4_decomposed(b, bone.matrix)

for path in Path("assets").glob("**/*.fbx"):
    start = time.time()
    
    settings = Settings(path.parent.joinpath("settings.json"))
    if settings.animations != True: continue

    clear_scenes()
    bpy.ops.import_scene.fbx(filepath=str(path))

    b = bytearray(b"A")

    bpy.ops.object.mode_set(mode='POSE')
    set_last_frame()
    export_frames(b)
    
    open(path.with_suffix(".bin"), "wb+")\
        .write(zstd.ZSTD_compress(bytes(b), settings.compression_level))

    print(f"{str(path)} compiled in {(time.time() - start):.2f} seconds\n")

print(f"Blender script finished in: {(time.time() - script_start):.2f} seconds\n")