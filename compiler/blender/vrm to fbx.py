import bpy
from pathlib import Path

names = [
    "Root",
    "J_Sec_L_Bust1",
    "J_Sec_L_Bust2",
    "J_Sec_R_Bust1",
    "J_Sec_R_Bust2",
    "J_Adj_L_FaceEye",
    "J_Adj_R_FaceEye"
]

def clear_scene():
    for obj in bpy.context.scene.objects: obj.select_set(True)
    bpy.ops.object.delete()

def export(path: Path, us = False):
    bpy.ops.export_scene.fbx(
        filepath=str(path),
        use_triangles = True,
        use_armature_deform_only = True,
        add_leaf_bones = False,
        object_types = {"ARMATURE", "MESH"},
        use_selection = us
    )

for path in Path("assets").glob("**/*.vrm"):
    bpy.ops.wm.read_homefile()
    clear_scene()
    bpy.ops.import_scene.vrm(filepath=str(path))

    for ob in bpy.context.scene.objects:
        if ob.type != "ARMATURE": continue

        bpy.ops.object.mode_set(mode='OBJECT')
        bpy.ops.object.select_all(action='DESELECT')
        bpy.context.view_layer.objects.active = ob
        bpy.ops.object.mode_set(mode='EDIT')

        armature = ob
        
        for name in names:
            ob.data.edit_bones.remove(ob.data.edit_bones[name])
        
        break

    bpy.ops.object.mode_set(mode='OBJECT')
    
    for ob in bpy.context.scene.objects:
        if ob.type != "MESH": continue

        bpy.ops.object.select_all(action='DESELECT')
        bpy.context.view_layer.objects.active = ob

        for name in names:
            ob.vertex_groups.remove(ob.vertex_groups.get(name))

bpy.ops.export_scene.fbx(
    filepath=str(path.with_suffix('.fbx')),
    use_triangles = True,
    object_types = {"ARMATURE", "MESH"}
)