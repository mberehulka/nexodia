{
    cargo run --bin compiler
    wait
}&
{
    blender --factory-startup --background --python ./compiler/blender/compile.py | grep -e "^assets" -e "finished"
    # blender --factory-startup --background --python ./compiler/blender/compile.py
    wait
}&

wait

read -p "Compilation finished in $SECONDS seconds, press enter to exit..."