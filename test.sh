python3 -m maturin build  
echo ""
echo "------------------" 
echo ""
python3  -m pip install target/wheels/go_gym-0.1.0-cp311-cp311-macosx_11_0_arm64.whl --force-reinstall
echo ""
echo "------------------"
echo ""
python3 python/tests/test_go_env.py
