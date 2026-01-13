gate mygate(theta) q {
    rz(theta) q;
    x q;
}
gate cx a,b {
    ctrl @ x a,b;
}
